use anyhow::Result;
use log;

// IDF
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::peripheral,
    wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};

// Create a new WiFi connection.
// It also binds
pub fn new<'a>(
    ssid: &str,
    pass: &str,
    modem: impl peripheral::Peripheral<P=esp_idf_svc::hal::modem::Modem> + 'a,
    sysloop: EspSystemEventLoop,
    // Result will live as long as `modem` lives
) -> Result<EspWifi<'a>> {
    // Init WiFi. Bind network to it.
    // This binds the IDF implementation of OSI Layer 2 (the "Data Link" layer that sends/receives ethernet packets)
    // to standard Rust networking: as a result, you can use network functions.
    // Also, we wrap it into a strictly blocking wrapper: internally, it waits for events (using the event loop)
    // and keeps polling is_connected(), etc.
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    // Start WiFi and scan for networks.
    // We start with blank configuration. Will set the SSID later.
    log::info!("Starting wifi...");
    wifi.set_configuration(&Configuration::Client(ClientConfiguration{
        // Fast scan. Find one SSID only.
        // ssid: ssid.try_into().unwrap(),
        // scan_method: esp_idf_svc::wifi::ScanMethod::FastScan,
        ..Default::default()
    }))?;
    wifi.start()?;
    log::info!("Scanning...");
    let ap_infos = wifi.scan()?;

    // Find our network: using SSID match.
    // If found, read its channel & auth method.
    // If not, we'll proceed with unknown channel
    let found_ap = ap_infos.into_iter().find(|a| a.ssid == ssid);
    let (channel, auth_method) = if let Some(ap) = found_ap {
        // let formattedBssid = {
        //     let [a, b, c, d, e, f] = ap.bssid;
        //     format!("{a:02X}:{b:02X}:{c:02X}:{d:02X}:{e:02X}:{f:02X}")
        // };
        // log::info!("AP found: {ssid} on channel {} signal={} auth={:?} bssid={}",
        //     ap.channel, ap.signal_strength, ap.auth_method, formattedBssid);
        (Some(ap.channel), ap.auth_method)
    } else {
        log::info!("AP not found: {ssid}. Will go with defaults.");
        (None, None)
    };

    // Set configuration now and connect.
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),    // converts into heapless::String
        password: pass.try_into().unwrap(),
        // Use auth method from the AP.
        // If not found, use None (if no password) or WPA2 (good default)
        auth_method: auth_method.unwrap_or({
            if pass.is_empty() { AuthMethod::None } else { AuthMethod::WPA2Personal }
        }),
        channel,
        //  ESP-IDF tries to register the hostname "espressif" in your local network,
        // so often http://espressif/ will work. You can customize the hostname using `sdkconfig.defaults`:
        // CONFIG_LWIP_LOCAL_HOSTNAME="esp32c3"
        // Docs: https://docs.espressif.com/projects/esp-idf/en/v5.5.1/esp32c3/api-reference/kconfig-reference.html
        ..Default::default()
    }))?;

    // Connect
    log::info!("Connecting wifi...");
    wifi.connect()?;
    log::info!("Waiting for DHCP lease...");
    wifi.wait_netif_up()?;
    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    log::info!("Got DHCP: {:?}", ip_info);

    // Done
    Ok(esp_wifi)
    // Ok(Box::new(esp_wifi))
}
