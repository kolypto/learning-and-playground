// Cargo add:
// $ cargo add toml-cfg anyhow esp-idf-hal
use anyhow::Result;
use std::time::Duration;

// IDF HAL and services
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{self, OutputMode, PinDriver},
    prelude::*,
    temp_sensor
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
};

// Our libraries
use a04_std_idf_http_client::{
    blinky_led,
    wifi,
};

// Drivers initialized on peripherals
// Lifetime 'd is the "device lifetime" (conventionally)
struct Drivers<'d,
    // It's nice to have all the pinout in one place, isn't it?
    StatusLedPin: gpio::Pin = gpio::Gpio8,
    RedLedPin:    gpio::Pin = gpio::Gpio0,
    GreenLedPin:  gpio::Pin = gpio::Gpio2,
    BlueLedPin:   gpio::Pin = gpio::Gpio1,
> {
    status_led: blinky_led::BlinkyLed<'d, StatusLedPin, gpio::Output>,
    modem:      esp_idf_hal::modem::Modem,
    internal_temp: esp_idf_hal::temp_sensor::TempSensorDriver<'d>,
    // NOTE: The three diodes have a common Anode (+), and their Cathodes (-) are connected to pins.
    // This is called Active-LOW or Inverted Logic: the GPIO pin is now acting as a current sink.
    // Therefore, set the pin to LOW for the current to flow into the pin sink.
    red_led:    blinky_led::BlinkyLed<'d, RedLedPin, gpio::Output>,
    green_led:  blinky_led::BlinkyLed<'d, GreenLedPin, gpio::Output>,
    blue_led:   blinky_led::BlinkyLed<'d, BlueLedPin, gpio::Output>,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let sysloop = EspSystemEventLoop::take()?;
    let _nvs = EspDefaultNvsPartition::take()?;

    // Peripherals
    let config = CONFIG;
    let mut drv = Drivers::new()?;

    let temp = drv.internal_temp.get_celsius()?;
    log::info!("Internal temp: {temp} °C");

    // WiFi
    // TODO: move into `drv`
    log::info!("Board starting...");
    drv.status_led.blink(1, Duration::from_millis(100), Duration::from_millis(200))?;
    let _wifi = wifi::new(config.wifi_ssid, config.wifi_psk, drv.modem, sysloop)?;
    drv.status_led.blink(3, Duration::from_millis(100), Duration::from_millis(200))?;

    // Init HTTP server
    let _server = a05_std_idf_http_server::http_server::new(
        drv.internal_temp,
        drv.red_led,
        drv.green_led,
        drv.blue_led,
    )?;
    println!("Server awaiting connection");

    loop {
        FreeRtos::delay_ms(100);
    }
}


impl <'d> Drivers<'d> {
    fn new() -> Result<Self> {
        // Take the peripherals.
        // Note that the value is moved, and it was a singleton.
        // Now no one else can use the peripherals: only through us.
        let peripherals = esp_idf_hal::peripherals::Peripherals::take()?;

        // Configure
        let mut status_led_output = PinDriver::output(peripherals.pins.gpio8)?;
        let mut red_led_output = PinDriver::output(peripherals.pins.gpio0)?;
        let mut green_led_output = PinDriver::output(peripherals.pins.gpio2)?;
        let mut blue_led_output = PinDriver::output(peripherals.pins.gpio1)?;
        let mut internal_temp_sensor = temp_sensor::TempSensorDriver::new(&temp_sensor::TempSensorConfig::default(), peripherals.temp_sensor)?;

        // Drivers
        Ok(Self {
            status_led: blinky_led::BlinkyLed::new(status_led_output)?,
            modem: peripherals.modem,
            red_led: blinky_led::BlinkyLed::new_inverted(red_led_output)?,
            green_led: blinky_led::BlinkyLed::new_inverted(green_led_output)?,
            blue_led: blinky_led::BlinkyLed::new_inverted(blue_led_output)?,
            internal_temp: {
                internal_temp_sensor.enable()?;
                internal_temp_sensor
            },
        })
    }
}


#[toml_cfg::toml_config]
pub struct Config { // cfg.toml
    #[default("Wokwi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}
