use defmt;
use esp_hal::{
    rng::Rng,
};
use crate::make_static;

use embassy_executor::Spawner;
use esp_radio::wifi;
use embassy_time::{Duration, Timer};
use core::{net::Ipv4Addr, str::FromStr};
use embassy_net::{DhcpConfig, Ipv4Cidr, Stack, StaticConfigV4};


// anyhow: return errors
use anyhow::{Context, Result, anyhow};

// Run me:
// $ SSID="name" PASSWORD="secret" cargo run
// $ SSID="name" PASSWORD="secret" STATIC_IP=192.168.0.199/24 GATEWAY=192.168.0.1 cargo run
// $ AP_MODE=1 SSID="esp32" PASSWORD="12345678" STATIC_IP=192.168.12.1/24 GATEWAY=192.168.12.1 cargo run


// Load WiFi credential from environment variables
// Variables will be read *at compile time*
const AP_MODE: Option<&str> = option_env!("AP_MODE");  // run in AP mode
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");
const STATIC_IP: Option<&str> = option_env!("STATIC_IP"); // optional
const GATEWAY_IP: Option<&str> = option_env!("GATEWAY"); // optional

// The number of sockets to allocate enough space for.
const N_SOCKETS: usize = 7;

// Start WiFi, spawn net tasks, return net stack
pub async fn start_wifi(
    spawner: &Spawner,
    wifi_peripheral: esp_hal::peripherals::WIFI<'static>,
) -> Result<Stack<'static>> {
    // Init Wifi.
    // Make a static variable: it's globally available.
    let radio_init = &*make_static!(
        esp_radio::Controller<'static>,
        esp_radio::init()
            // Use .context() to wrap the error
            .context("Failed to initialize the radio")?
            // .map_err(|e| anyhow!("Failed to initialize the radio: {}", e))?
    );

    // Init controller
    let (wifi_controller, interfaces) =
        wifi::new(&radio_init, wifi_peripheral, Default::default())
            .context("Failed to initialize Wi-Fi controller")?;
    let wifi_interface = if AP_MODE.is_none() {
        interfaces.sta // WiFi station: the client
    } else {
        interfaces.ap
    };

    // Network stack needs a random number: for TLS and networking.
    // The net stack wants a u64, so we join two u32-s.
    let rng = Rng::new();
    let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    // Init network stack
    let net_config = {
        // Static ip
        if let Some(ip) = STATIC_IP && let Some(gw) = GATEWAY_IP {
            embassy_net::Config::ipv4_static(StaticConfigV4{
                address: Ipv4Cidr::from_str(ip).unwrap(),
                gateway: Ipv4Addr::from_str(gw).ok(),
                dns_servers: Default::default(), // TODO: use Google DNS by default?
            })
        // DHCP
        } else {
            embassy_net::Config::dhcpv4({
                let mut c = DhcpConfig::default();
                c.hostname = Some(heapless::String::new());
                c
            })
        }
    };
    let (stack, runner) = embassy_net::new(
        wifi_interface, net_config,
        // Stack resources: size=3
        make_static!(embassy_net::StackResources<N_SOCKETS>, embassy_net::StackResources::<N_SOCKETS>::new()),
        net_seed,
    );

    // Start two background tasks:
    // - the connection_task will maintain the Wi-Fi connection
    // - the net_task will run the network stack and handle network events.
    if AP_MODE.is_none() {
        spawner.spawn(task_keep_wifi_client_up(wifi_controller)).ok();
    } else {
        spawner.spawn(task_keep_wifi_ap_up(wifi_controller)).ok();
    }
    spawner.spawn(task_network(runner)).ok();

    // Wait until the connection is up
    wait_for_connection(stack).await;

    // Done
    Ok(stack)
}


// Task: run the network stack
#[embassy_executor::task]
async fn task_network(mut runner: embassy_net::Runner<'static, wifi::WifiDevice<'static>>) {
    runner.run().await
}


// Task: manage WiFi connection by continuously checking the status, configuring the Wi-Fi controller,
// and attempting to reconnect if the connection is lost or not started.
#[embassy_executor::task]
async fn task_keep_wifi_client_up(mut controller: wifi::WifiController<'static>) {
    defmt::info!("WiFi: start client");
    defmt::info!("WiFi: Device capabilities: {:?}", controller.capabilities());

    loop {
        // 1. Check WiFi state
        // If it is in StaConnected, we wait until it gets disconnected.
        match wifi::sta_state() {
            wifi::WifiStaState::Connected => {
                // wait until we're no longer connected
                controller.wait_for_event(wifi::WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await;
            }
            _ => {},
        }

        // 2. Check if the WiFi controller is started.
        // If not, we initialize the WiFi client configuration.
        if !matches!(controller.is_started(), Ok(true)) {
            // Init client. Use SSID.
            let client_config = wifi::ModeConfig::Client(
                wifi::ClientConfig::default()
                    .with_ssid(SSID.into())
                    .with_password(PASSWORD.into())
                    .with_auth_method(wifi::AuthMethod::Wpa2Personal),  // TODO: configurable?
            );
            controller.set_config(&client_config).unwrap();
            defmt::debug!("WiFi: starting...");

            // Wifi start.
            controller.start_async().await.unwrap();
        }

        // Wait until connected
        defmt::debug!("WiFi: connecting...");
        match controller.connect_async().await {
            // NOTE: This is only WiFi.
            // The network stack (smoltcp) will need to use its DHCP client now.
            Ok(_) => {
                let rssi = controller.rssi().unwrap_or(-999);
                defmt::info!("WiFi: connected! rssi={}", rssi);
            }
            Err(e) => {
                defmt::warn!("WiFi: failed to connect: {:?}", e);

                // Sleep 5s before trying again
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

// Task: wait for the Wi-Fi link to be up, then obtain the IP address.
async fn wait_for_connection(stack: embassy_net::Stack<'_>) {
    let mut print_once: bool = false;
    while !stack.is_link_up() {
        if !print_once {
            defmt::debug!("Net: waiting on link ...");
            print_once = true;
        }
        Timer::after(Duration::from_millis(100)).await;
    }

    let mut print_once: bool = false;
    // while !stack.is_config_up() {
    //     Timer::after(Duration::from_millis(100)).await
    // }
    loop {
        if let Some(config) = stack.config_v4() {
            defmt::info!("IP: {}", config.address);
            break;
        }
        if !print_once {
            defmt::debug!("Net: waiting for network config ...");
            print_once = true;
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}





#[embassy_executor::task]
async fn task_keep_wifi_ap_up(mut controller: wifi::WifiController<'static>) {
    defmt::info!("WiFi: start AP");

    loop {
        // WiFi AP is up and running? Then do nothing.
        if wifi::ap_state() == wifi::WifiApState::Started {
            Timer::after(Duration::from_millis(1000)).await;
            continue;
        }

        // Re-configure AP
        if !matches!(controller.is_started(), Ok(true)) {
            // Init client. Use SSID.
            let ap_config = wifi::ModeConfig::AccessPoint(
                wifi::AccessPointConfig::default()
                    .with_ssid(SSID.into())
                    .with_password(PASSWORD.into())
                    .with_auth_method(wifi::AuthMethod::Wpa2Wpa3Personal),
            );
            controller.set_config(&ap_config).unwrap();
            defmt::info!("WiFi: Starting ...");

            // Wifi start.
            controller.start_async().await.unwrap();
        }
    }
}
