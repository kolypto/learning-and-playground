#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
esp_bootloader_esp_idf::esp_app_desc!();
use {esp_backtrace as _, esp_println as _};

// This time we'll use defmt
use defmt;
use esp_hal::{
    delay,
    gpio,
    main,
};

#[main]
fn main() -> ! {
    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // PIR Sensor.
    // With an initial state of "pull down": it goes HIGH when motion is detected.
    let pir_sensor = gpio::Input::new(peripherals.GPIO0, gpio::InputConfig::default()
        .with_pull(gpio::Pull::Down));

    let delay = delay::Delay::new();
    loop {
        if pir_sensor.is_high() {
            defmt::info!("Motion detected");
            // TODO: You can trigger the buzzer here.
            delay.delay_millis(100);
        }
        delay.delay_millis(100);
    }
}
