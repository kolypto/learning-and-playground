#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]

use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();

use log;
use esp_hal::{
    time::{Instant, Duration},
    clock::CpuClock,
    gpio, analog::adc,
    delay::Delay,
    main,
};
use nb;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // LED
    let mut onboard_led = gpio::Output::new(peripherals.GPIO8, gpio::Level::Low, gpio::OutputConfig::default());

    // ADC Input
    // Attenuation: -11dB
    let ldr_pin = peripherals.GPIO0;
    let mut adc_config = adc::AdcConfig::new();
    let mut pin = adc_config.enable_pin(ldr_pin, adc::Attenuation::_11dB);
    let mut adc1 = adc::Adc::new(peripherals.ADC1, adc_config);

    let delay = Delay::new();
    loop {
        // Read
        // NOTE: ADC would sometimes return Err(WouldBlock). This we ignore.
        match adc1.read_oneshot(&mut pin) {
            Ok(result) => log::info!("adc={result}"),
            Err(nb::Error::WouldBlock) => continue,
            Err(err) => log::error!("err={err:?}"),
        }

        // Instead, we can use nb::block!() to wait until the value becomes available.
        // Internally, it loop{}s until WouldBlock goes away and a value becomes available.
        let luminosity: u16 = match nb::block!(adc1.read_oneshot(&mut pin)) {
            Ok(result) => {
                (result.max(1900)-1900)/20
            }
            Err(err) => {
                log::error!("err={err:?}");
                continue;
            }
        };
        log::info!("luminosity={luminosity}");

        // Blink LED
        let now = Instant::now();
        while now.elapsed() < Duration::from_millis(300) {
            onboard_led.toggle();
            delay.delay_millis(luminosity as u32);
        }
    }
}
