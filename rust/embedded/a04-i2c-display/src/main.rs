#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    prelude::*,
    clock::ClockControl, peripherals::Peripherals,
    Delay,
    i2c::I2C, IO,
};
use core::fmt::Write;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{
    prelude::*,
    I2CDisplayInterface, Ssd1306,
    mode::BufferedGraphicsMode,  mode::TerminalMode,
};



#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let mut io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Create a new I2C peripheral
    let mut i2c = I2C::new(
        peripherals.I2C0, // we have 2 I2C peripherals
        io.pins.gpio21,  // SDA pin to use
        io.pins.gpio22,  // SCL pin to use
        100_u32.kHz(),  // frequency. 100kHz is the "standard mode"
        &mut system.peripheral_clock_control,
        &clocks,
    );


    // let interface = I2CDisplayInterface::new(i2c);
    // let mut display = Ssd1306::new(
    //     interface,
    //     DisplaySize128x64,
    //     DisplayRotation::Rotate0,
    // ).into_buffered_graphics_mode();
    // display.init().unwrap();

    // let text_style = MonoTextStyleBuilder::new()
    //     .font(&FONT_6X10)
    //     .text_color(BinaryColor::On)
    //     .build();

    // Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
    //     .draw(&mut display)
    //     .unwrap();

    // Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
    //     .draw(&mut display)
    //     .unwrap();

    // display.flush().unwrap();


    // let interface = I2CDisplayInterface::new(i2c);

    // let mut display = Ssd1306::new(
    //     interface,
    //     DisplaySize128x64,
    //     DisplayRotation::Rotate0,
    // ).into_terminal_mode();
    // display.init().unwrap();
    // display.clear().unwrap();

    // // Spam some characters to the display
    // for c in 97..123 {
    //     let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
    // }
    // for c in 65..91 {
    //     let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
    // }


    loop {
        // Get us a buffer and read into it
        let mut data = [0u8; 22];
        // i2c.write_read(<device_addr>, <register>, <buffer>)
        // i2c.write_read(DISPLAY_ADDR, &[0x00], &mut data).ok();
        println!("{:?}", data);

        delay.delay_ms(500_u32);
    }
}

const DISPLAY_ADDR: u8 = 0b111100;
const ACCELEROMETER_ADDR: u8 = 0x3A;
