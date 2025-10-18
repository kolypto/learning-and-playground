#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]
esp_bootloader_esp_idf::esp_app_desc!();
use {esp_backtrace as _, esp_println as _};
use defmt;

use esp_hal::{
    clock::CpuClock,
    time::{Duration, Instant, Rate},
    delay::Delay,
    i2c::master::{I2c, Config as I2cConfig},
    main,
};

// HD44780 Driver
// use hd44780_driver::{
//     HD44780,
//     // memory_map::MemoryMap1602,
//     // setup::DisplayOptionsI2C,
// };
use hd44780_driver::{
    setup::DisplayOptionsI2C,
    HD44780,
};

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut i2c = I2c::new(
            peripherals.I2C0,
            I2cConfig::default().with_frequency(Rate::from_khz(400)),
        ).expect("i2c bus init")
        .with_sda(peripherals.GPIO8)
        .with_scl(peripherals.GPIO9);

    // Scan I2C bus
    // - I2C uses 7-bit addresses (0x00-0x7F)
    // - Skip 0x00-0x02 (reserved for special protocols)
    // - Skip 0x78-0x7F (10-bit addressing and reserved)
    for addr in 0x03..=0x77 {
        match i2c.write(addr, &[]) {
            Ok(_) => {
                defmt::info!("Device found at 0x{:02X}", addr);
            }
            Err(_) => {
                // No device here, NACK received
                // NACK is assumed if no device pulls the line LOW.
            }
        }
    }

    // Init display.
    // I2C address = 0x27
    let i2c_address = 0x27;
    let mut delay = Delay::new();
    let mut options = DisplayOptionsI2C::new(
        // Memory map: 16x2 characters
        hd44780_driver::memory_map::MemoryMap1602::new()
    ).with_i2c_bus(i2c, i2c_address);

    let mut display = loop {
		match HD44780::new(options, &mut delay) {
			Err((options_back, error)) => {
				defmt::error!("Error creating LCD Driver: {:?}", defmt::Debug2Format(&error));
				options = options_back;
				delay.delay_millis(500);
				// try again
			}
			Ok(display) => break display,
		}
	};

    // Unshift display and set cursor to 0
    // Then clear existing characters
	display.reset(&mut delay).unwrap();
    display.clear(&mut delay).unwrap();
	display.write_str("31337", &mut delay).unwrap();

    loop {
        delay.delay_millis(500);
    }
}
