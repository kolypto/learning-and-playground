// Configuration.
// Is picked up *at compile time* by `build.rs` from `cfg.toml`
// $ cargo add toml-cfg
// This macros defines variable: CONFIG
#[toml_cfg::toml_config]
pub struct Config { // cfg.toml
    #[default("Wokwi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

use std::time::Duration;

// IDF HAL and services
use esp_idf_hal::{
    prelude::*,
    delay::FreeRtos,
    gpio::PinDriver,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
};

// Provides anyhow::Error for easy error handling:
//   anyhow::Result<T> = anyhow::Result<T, Error>
//   bail!("Missing attribute: {}", missing); = return Err(anyhow!(..));
//   ensure!(user == 0, "only user 0 is allowed");
use anyhow::{bail, Result};

// Our libraries
use a04_std_idf_http_client::{
    blinky_led, http_client, rgb_led::{self, RGB8}, wifi
};

// App: will connect to WiFi, blink the LED green/red to show the outcome.
// Will blink yellow while connecting.
fn main() -> Result<()> {
    // Required. Makes sure that patches to ESP-IDF are linked to the final executable.
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // We're running on top of FreeRTOS now.
    let sysloop = EspSystemEventLoop::take()?;

    // Init NVS (non-volatile storage).
    // WiFi driver uses it to store calibration data.
    // Without it, WiFi can't optimize its radio performance.
    let _nvs = EspDefaultNvsPartition::take()?;

    // Peripherals/devices/components
    let peripherals = Peripherals::take().unwrap();
    let blue_led_pin = peripherals.pins.gpio8;
    let rgb_led_pin = peripherals.pins.gpio4;  // addressable LED
    let rgb_led_rmt0 = peripherals.rmt.channel0;  // generate signals (for RGB LEDs)

    // Driver: Simple GPIO LED
    // In IDF, we use `PinDriver` to control the pin. Not the bare-metal `Output`.
    let mut blue_led_output = PinDriver::output(blue_led_pin)?;
    blue_led_output.set_low()?;
    let mut blink = blinky_led::BlinkyLed::new(blue_led_output)?;

    // Driver: RGB LED
    let mut led = rgb_led::WS2812RMT::new(rgb_led_pin, rgb_led_rmt0)?;

    // Log
    log::info!("Board starting...");

    //=== RGB LED control ===//

    // LED: blink yellow.
    // Use our module: rgb_led
    for _ in 0..3 {
        // NOTE: This is inefficient. WS2812Z protocol supports animations.
        led.set_pixel(RGB8::new(50, 50, 0))?;

        // Sleep using FreeRTOS.
        // It hopefully uses proper wait.
        FreeRtos::delay_ms(200);

        // Turn off
        led.set_pixel(RGB8::new(0, 0, 0))?;
        FreeRtos::delay_ms(200);
    }

    // LED: Control a 64x64 LED matrix of addressable LEDsw
    log::info!("LED Heart 64x64");
    led.heart64x64()?;

    //=== WiFI connect ===//

    // Get CONFIG: the variable is set by `toml_cfg`
    let app_config = CONFIG;

    // Connect to WiFi
    // Connect to the Wi-Fi network
    let _wifi = match wifi::new(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        // The modem peripheral
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => inner,
        Err(err) => {
            // Red!
            led.set_pixel(RGB8::new(50, 0, 0))?;
            bail!("Could not connect to Wi-Fi network: {:?}", err);

            // We panicked.
            // ESP32 will restart and try again.
        }
    };

    // Yield control to FreeRTOS.
    // Otherwise it might think we're dead.
    FreeRtos::delay_ms(0);

    //=== HTTP Load Page ===//

    // We'll keep re-loading the page.
    // Every time it changes (the content length) — we blink.

    let mut last_content_len: u64 = 0;

    loop {
        const URL: &str = "https://example.com/";

        // HTTP Client.
        // Load a webpage.
        let (content_len, body )= http_client::get_page(URL)?;
        log::info!("HTTP GET: Content-Length={content_len} Body={body}");

        // Blink if content length changed
        if last_content_len != content_len {
            blink.blink(6, Duration::from_millis(300), Duration::from_millis(100))?;
        }
        last_content_len = content_len;

        // We sleep; RTOS runs other tasks.
        // WARNING: if we do not yield control for >5s, the following error pops up:
        //  E (21573) task_wdt: Task watchdog got triggered. The following tasks/users did not reset the watchdog in time:
        //  E (21573) task_wdt:  - IDLE (CPU 0)
        //  E (21573) task_wdt: CPU 0: main
        FreeRtos::delay_ms(1000); // let other tasks run
    }
}
