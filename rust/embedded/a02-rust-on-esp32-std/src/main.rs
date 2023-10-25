use esp_idf_hal::{
    prelude::Peripherals,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
};
use anyhow::{bail, Result};
use core::str;
use log;

use a02_rust_on_esp32_std::{wifi, httpclient}; // our lib

fn main() -> Result<()> {
    // Need to call this once: applies patches to the runtime.
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Let HAL take the peripherals
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // Print something
    println!("Started :)");

    // Connect to WiFi
    let _wifi = match wifi::connect(
        CONFIG.wifi_ssid,
        CONFIG.wifi_psk,
        peripherals.modem,
        sysloop,
    ){
        Ok(wifi) => {
            log::info!("Connected to Wi-Fi network {:?}!", CONFIG.wifi_ssid);
            wifi
        }
        Err(err) => {
            // Red!
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    // HTTP request
    httpclient::get_url("https://api.myip.com/")?;

    Ok(())
}


// App config. Auto-generated as `CONFIG`
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}
