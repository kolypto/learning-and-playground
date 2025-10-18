#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]
esp_bootloader_esp_idf::esp_app_desc!();
use esp_println as _;
use esp_backtrace as _;
use defmt::info;

use esp_hal::{
    clock::CpuClock,
    time::Rate,
    timer::timg::TimerGroup,
    spi,
    gpio,
};

use embassy_executor::Spawner;
use embassy_time::{Timer, Delay};

use epd_waveshare::{
    prelude::*,
    epd2in13_v2::{Epd2in13, Display2in13},
    color::Color,
};
use embedded_graphics::{
    prelude::*,
    Drawable,
    mono_font,
    text,
    image,
    primitives,
};

// TinyBMP: no-std, low-memory BMP loader
// $ cargo add tinybmp
use tinybmp;

#[allow(clippy::large_stack_frames)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt = esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    /* eInk display
     * Pins: 6: MOSI ; 4: SCK ; 7: CS ; 8: DC ; 20: RST; 21: BUSY
     * - CS: active-LOW, initially HIGH
     * - DC: HIGH = pixel data, LOW = commands
     * - RST: active-LOW, initially HIGH. To reset, pull LOW then HIGH
     * - BUSY: input, active-HIGH. LOW when idle, HIGH when busy
     */

    // Init SPI bus.
    let spi_bus = spi::master::Spi::new(
        peripherals.SPI2,
        spi::master::Config::default()
            .with_frequency(Rate::from_khz(400))  // 4Mhz should be fine
            .with_mode(spi::Mode::_0),
        ).expect("init SPI")
        .with_sck(peripherals.GPIO4)
        .with_mosi(peripherals.GPIO6) // MOSI. No MISO.
        ;
    let cs = gpio::Output::new(peripherals.GPIO7, gpio::Level::High, gpio::OutputConfig::default());

    // Convert it into an embedded-hal-bus device
    let mut spi_dev = embedded_hal_bus::spi::ExclusiveDevice::new(spi_bus, cs, Delay).expect("init SPI device");

    // Give eInk display some time to init
    Timer::after_millis(100).await;

    // Init display
    defmt::info!("Init display");
    let dc = gpio::Output::new(peripherals.GPIO8, gpio::Level::Low, gpio::OutputConfig::default());
    let reset = gpio::Output::new(peripherals.GPIO20, gpio::Level::High, gpio::OutputConfig::default());
    let busy_in = gpio::Input::new(peripherals.GPIO21, gpio::InputConfig::default().with_pull(gpio::Pull::Down));
    let mut epd = Epd2in13::new(&mut spi_dev, busy_in, dc, reset, &mut Delay, None).expect("init eInk");
    let mut display = Display2in13::default();
    display.set_rotation(DisplayRotation::Rotate90);

    // Clear display:
    // - Clear its internal buffer
    // - Fill the display with white
    // - Send changes to the display
    defmt::info!("Clear display");
    epd.clear_frame(&mut spi_dev, &mut Delay).expect("clear display");
    display.clear(Color::White).expect("display clear");
    epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay).expect("epd update");

    // We can wait until idle -- but there's no need: all functions do it internally.
    // One problem, though: they're not async :(
    epd.wait_until_idle(&mut spi_dev, &mut Delay).expect("wait until idle");

    // Display image
    let bmp_data = include_bytes!("../../misc/xakep.bmp");
    let bmp = tinybmp::Bmp::from_slice(bmp_data).unwrap();
    let image = image::Image::new(&bmp, Point::new(0, 0));
    image.draw(&mut display).expect("draw image");

    // Get image size
    use embedded_graphics::geometry::OriginDimensions; // adds: .size()
    let text_y = bmp.size().height as i32;

    // Write text below the image
    defmt::info!("Writing text");
    text::Text::with_baseline("31337", Point::new(3, text_y), MONO_TEXT_STYLE, text::Baseline::Top)
        .draw(&mut display)
        .expect("write text");


    // Draw a circle
    primitives::Circle::new(Point::new(150, text_y), 40)
        .into_styled(primitives::PrimitiveStyle::with_stroke(Color::Black, 2))
        .draw(&mut display)
        .ok();

    // Flush image buffer: all at once
    epd.update_and_display_frame(&mut spi_dev, display.buffer(), &mut Delay).expect("flush image buffer");

    // VERY IMPORTANT:
    // put it into sleep mode or completely power it off after use!
    // Otherwise, you'll end up damaging the display.
    epd.sleep(&mut spi_dev, &mut Delay).expect("display sleep");

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        info!("Hello world!");
        Timer::after_secs(1).await;
    }
}

// Font style
static MONO_TEXT_STYLE: mono_font::MonoTextStyle<'_, Color> = mono_font::MonoTextStyleBuilder::new()
    .font(&mono_font::ascii::FONT_10X20)
    .text_color(Color::Black)
    // .background_color(Color::White)
    .build();

// We need a B/W image:
// $ convert img.png -monochrome img.bmp
// static img: tinybmp::Bmp<'_, Color> = tinybmp::Bmp::<'_, Color>::from_slice(include_bytes!("../../misc/ferris.bmp")).unwrap();
