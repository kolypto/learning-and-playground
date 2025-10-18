#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
esp_bootloader_esp_idf::esp_app_desc!();
use {esp_backtrace as _, esp_println as _};

use defmt::info;
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    i2c::{self, master::I2c},
    time::Rate,
    Async,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Instant};

// SSD1306 display driver
use ssd1306;

// Embedded graphics: 2D graphics library.
// Can draw or print text on anything that implemements the `DrawTarget` trait.
use embedded_graphics::{
    Drawable,
    image::{Image, ImageRaw},
    mono_font,
    pixelcolor,
    prelude::{Point, Size},
    text::{Baseline, Text}
};
// TinyBMP: no-std, low-memory BMP loader
// $ cargo add tinybmp
use tinybmp;

// Use it to share the I2C bus
// $ cargo add embassy-sync --features defmt
// $ cargo add embassy-embedded-hal --features defmt
use embassy_sync::{
    mutex::Mutex,
    blocking_mutex::raw::NoopRawMutex,
};
use embassy_embedded_hal::{
    shared_bus::asynch::i2c::I2cDevice,
};
use static_cell::{ StaticCell };

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // RTOS
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt = esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);



    // === Init Shared I2C Bus === //

    // I2C. Init in async mode.
    let i2c_bus = i2c::master::I2c::new(
        peripherals.I2C0,
        i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    ).expect("Init I2C bus").with_sda(peripherals.GPIO8).with_scl(peripherals.GPIO9).into_async();

    // We'll need to share the bus: 2 devices.
    // Make the bus static; init 2 shared buses
    static I2C_BUS: StaticCell<Mutex<NoopRawMutex, i2c::master::I2c<Async>>> = StaticCell::new();
    let i2c_shared_bus = I2C_BUS.init(Mutex::new(i2c_bus));

    // Create device handles for each peripheral
    let display_i2c = I2cDevice::new(i2c_shared_bus);
    let accelerometer_i2c = I2cDevice::new(i2c_shared_bus);




    // === Spawn Tasks === //
    spawner.spawn(task_accelerometer_jump_detection(accelerometer_i2c)).unwrap();




    // === Init Display === //

    // Init SSD1306 driver. Pass i2c bus to it.
    let display_iface = ssd1306::I2CDisplayInterface::new(display_i2c);
    let mut display = ssd1306::Ssd1306Async::new(
        display_iface,
        ssd1306::size::DisplaySize128x64,
        ssd1306::rotation::DisplayRotation::Rotate0,
    );
    display.init_with_addr_mode(ssd1306::command::AddrMode::Page).await.expect("Display init failed");


    // === Screen 1. Print ". . .". Display in "Terminal mode". === //

    // "Terminal Mode". Can only print text.
    let mut display = display.into_terminal_mode();

    // Clear the display first. Also resets the cursor to the top left corner.
    display.clear().await.expect("Clear");

    // Write ". . ."
    display.write_str("\n\n").await.expect("Text");
    for _ in 0..5 {
        Timer::after(Duration::from_millis(300)).await;
        display.write_str(" .").await.unwrap();
    }

    // Wait
    Timer::after(Duration::from_millis(300)).await;



    // === Screen 2. Print "No Internet". Display in "Graphics mode". === //

    // "Buffered Graphics Mode". Supports rich drawing features using `embedded_graphics`.
    let mut display = display.into_buffered_graphics_mode();
    display.init_with_addr_mode(ssd1306::command::AddrMode::Horizontal).await.expect("Display init failed");
    display.clear_buffer();  // clear the screen

    // Init text style
    let text_style = mono_font::MonoTextStyleBuilder::new().
        font(&mono_font::ascii::FONT_6X13).
        text_color(pixelcolor::BinaryColor::On).
        background_color(pixelcolor::BinaryColor::Off).
        build();

    // Print characters one by one
    let text = "No Internet.";
    for (i, c) in text.chars().enumerate() {
        let mut tmp = [0u8; 4];
        Text::new(
            c.encode_utf8(&mut tmp),
            Point::new(
                text_style.font.character_size.width as i32 * i as i32,
                3 * text_style.font.baseline as i32
            ),
            text_style
        ).draw(&mut display).expect("Print text");

        // Flush: write data to the display. Only then it gets updated.
        display.flush().await.unwrap();
        Timer::after(Duration::from_millis(200)).await;
    }

    // Clear screen
    Timer::after(Duration::from_secs(1)).await;
    display.clear_buffer();
    display.flush().await.unwrap();


    // === Game. Dino. === //

    // Prepare the Dino sprite
    // From array
    let _im_dino = ImageRaw::<pixelcolor::BinaryColor>::new(&DINO_SPRITE, DINO_SIZE.width);

    // From BMP
    let dino_l = tinybmp::Bmp::from_slice(include_bytes!("../../misc/dino-l.bmp")).unwrap();
    let dino_r = tinybmp::Bmp::from_slice(include_bytes!("../../misc/dino-r.bmp")).unwrap();
    let cactus = tinybmp::Bmp::from_slice(include_bytes!("../../misc/cactus.bmp")).unwrap();
    let cloud = tinybmp::Bmp::from_slice(include_bytes!("../../misc/cloud.bmp")).unwrap();
    use embedded_graphics::geometry::OriginDimensions; // adds: .size()

    // Sleep
    let now = Instant::now();
    let mut jump_in_progress: Option<Instant> = None;
    loop {
        // Clear display
        display.clear_buffer();

        // Dinosaur leg: left or right?
        let dino = if (now.elapsed().as_millis() / 400) % 2 == 0 {
            &dino_l
        } else {
            &dino_r
        };

        // Draw a cactus
        let x = display.size().width as i32 - (now.elapsed().as_millis() as u32/30 % (display.size().width + cactus.size().width)) as i32;
        let y = display.size().height as i32 - cactus.size().height as i32;
        let image = Image::new(&cactus, Point::new(x, y));
        image.draw(&mut display).unwrap();

        // Draw a cloud
        let x = display.size().width as i32 - (now.elapsed().as_millis() as u32/90 % (display.size().width + cloud.size().width)) as i32;
        let y = 0;
        let image = Image::new(&cloud, Point::new(x, y));
        image.draw(&mut display).unwrap();

        // Is the dino jumping?
        if jump_in_progress.is_none() &&
           let Some(jumped_at) = *LAST_JUMP_DETECTED.lock().await &&
           jumped_at.elapsed() < Duration::from_millis(200) {
                defmt::info!("Jump received: {}", jumped_at);
                // Use that instant, not the current one
                jump_in_progress = Some(jumped_at);
        }

        // Draw a dinosaur
        let mut y = display.size().height as i32 - dino.size().height as i32;
        if let Some(jump) = jump_in_progress {
            let t = (jump.elapsed().as_millis() / 100) as i32;
            let dy = 13 * t - t*t;
            if dy < 0 {
                jump_in_progress = None;
            } else {
                y -= dy;
            }
        }
        let image = Image::new(dino, Point::new(0, y));
        image.draw(&mut display).unwrap();

        // Draw
        display.flush().await.unwrap();

        // Sleep. Let others run.
        Timer::after(Duration::from_millis(30)).await;
    }
}




// Shared state: tells main() whether the user is jumping or not
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
};
static LAST_JUMP_DETECTED: Mutex<CriticalSectionRawMutex, Option<Instant>> = Mutex::new(None);

// Task: Accelerometer jump detection
#[embassy_executor::task]
async fn task_accelerometer_jump_detection(mut accelerometer_i2c: I2cDevice<'static, NoopRawMutex, I2c<'static, Async>>) -> ! {
    // Import the I2c trait. Otherwise the .write() method won't be accessible.
    use embedded_hal_async::i2c::I2c;  // .write()

    // === Init Accelerometer === //

    // Set up ADXL345
    const ADXL345_ADDR: u8 = 0x53;  // 0x53 if SDO is LOW, 0x1D if SDO is HIGH
    const REG_POWER_CTL: u8 = 0x2D;
    const REG_DATA_FORMAT: u8 = 0x31;
    const REG_DATAZ0: u8 = 0x36;  // Z-axis LSB
    const _REG_DATAZ1: u8 = 0x37;  // Z-axis MSB
    // Set measurement mode (disable standby)
    accelerometer_i2c.write(ADXL345_ADDR, &[REG_POWER_CTL, 0x08]).await.expect("ADXL power on");
    // Set data format: ±2g range, full resolution
    accelerometer_i2c.write(ADXL345_ADDR, &[REG_DATA_FORMAT, 0x08]).await.expect("ADXL set up");
    // Let it settle
    Timer::after_millis(10).await;

    // Keep checking the accelerometer: the user might be jumping.
    let mut last_jump_detected = Instant::now();
    loop {
        // Sacrifice some precision to keep the bus clean.
        Timer::after(Duration::from_millis(10)).await;

        // Don't detect jumps if a jump is already in progress.
        // This will also save us some Mutex locking. And some bus noise.
        // Alternatively, we could just sleep after detection. So much easier.
        if last_jump_detected.elapsed() < Duration::from_millis(100) {
            continue;
        }

        // Check the accelerometer. Is the user jumping?
        // Read two registers in one transaction.
        // This works because ADXL345 auto-increments the register address during multi-byte reads.
        let mut data = [0u8; 2];
        accelerometer_i2c.write_read(ADXL345_ADDR, &[REG_DATAZ0], &mut data).await.unwrap();
        // Combine LSB and MSB into signed 16-bit value
        let raw = i16::from_le_bytes(data);
        // Convert the raw value into something readable.

        // Jumping: <50 or >500
        // This is the raw value. We could've converted it into real acceleration reading.
        if raw < 50 || raw > 500 {
            // Remember the jump detected time.
            // The main() thread can decide whether it's recent enough.
            last_jump_detected = Instant::now();
            defmt::info!("Jump detected!");
            *LAST_JUMP_DETECTED.lock().await = Some(last_jump_detected);
        }
    }
}






// 'dino-small-inv', WxH Pixel = 20 x 22 px
const DINO_SIZE: Size = Size::new(20, 22);
#[rustfmt::skip]
const DINO_SPRITE: [u8; 66] = [
    // Generated with: https://implferris.github.io/image2bytes/
    // Uses bit array
	0x00, 0x1f, 0xe0, 0x00, 0x3f, 0xf0, 0x00, 0x37, 0xf0, 0x00, 0x3f, 0xf0, 0x00, 0x3f, 0xf0, 0x00,
	0x3f, 0xf0, 0x00, 0x3e, 0x00, 0x00, 0x3f, 0xc0, 0x80, 0x7c, 0x00, 0x80, 0xfc, 0x00, 0xc3, 0xff,
	0x00, 0xe7, 0xfd, 0x00, 0xff, 0xfc, 0x00, 0xff, 0xfc, 0x00, 0x7f, 0xf8, 0x00, 0x3f, 0xf8, 0x00,
	0x1f, 0xf0, 0x00, 0x0f, 0xe0, 0x00, 0x07, 0x60, 0x00, 0x06, 0x20, 0x00, 0x04, 0x20, 0x00, 0x06,
	0x30, 0x00
];

