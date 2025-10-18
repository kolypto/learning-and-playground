#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

// Read constants from env at compile time (!)
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

use esp_backtrace as _;
use esp_println::{print, println};
use esp_hal::{
    clock::CpuClock,
    time::{self, Duration, Instant},
    main,
    // TimerGroup for esp-rtos (required by esp-radio)
    timer::timg::TimerGroup,
    // For the heap allocator
    ram,
    // Random
    rng::Rng,
};

// ESP WiFi
use esp_radio::wifi::{ClientConfig, AuthMethod, ModeConfig, ScanConfig};



// smoltcp: A TCP/IP stack designed for bare-metal, real-time systems without a heap.
use smoltcp::{
    self,
    iface::{SocketSet, SocketStorage},
    wire::{DhcpOption, IpAddress},
    socket::Socket,
};
use core::net::Ipv4Addr;

// Non-async Networking primitives for TCP/UDP communication
// This is basically just ripped out of esp-wifi.
// Why do we need to use this one?
// I don't know where the code from `esp-wifi` been moved. It's not in the `esp-radio`.
use blocking_network_stack::Stack;

// Import traits.
// Otherwise `socket` won't have `read()` and `write()` methods.
use embedded_io::{Read, Write};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();
extern crate alloc;

#[main]
fn main() -> ! {
    // Init hardware
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // esp-generator: init ESP heap allocator
    // ESP32 has fragmented RAM regions. Here we are manually carving out chunks for the allocator.
    // Multiple separate heap allocators:
    // - 64K in reclaimed RAM: memory freed after boot (was used by bootloader).
    // - 36KB in default DRAM
    // - 66K in uninitialized DRAM (section of Data RAM that is not initialized at boot)
    // Other available regions: rtc_fast, rtc_slow, persistent, zeroed, reclaimed
    esp_alloc::heap_allocator!(#[ram(reclaimed)] size: 64 * 1024);
    esp_alloc::heap_allocator!(size: 36 * 1024);
    // Commented out because my ESP32C3 does not have DRAM2
    // esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    // Prepare RTOS: esp-rtos. A minimal scheduler.
    // It provides executors to enable running async code, tasks,
    // and implements the necessary capabilities (threads, queues, etc.) required by esp-radio.
    // - Take ownership of a timer grup and a software interrupt
    // - Start RTOS scheduler using that timer and interrupt
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);


    //=== WiFi
    // Init WiFi
    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    let (mut wifi_controller, interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    // Disable power saving on WiFi
    wifi_controller
        .set_power_saving(esp_radio::wifi::PowerSaveMode::None)
        .expect("Failed to set WiFi power save mode = off");

    // Configure WiFi client
    let client_config = ModeConfig::Client(
        ClientConfig::default()
            .with_ssid(SSID.into())
            .with_password(PASSWORD.into())
            .with_auth_method(AuthMethod::None),
    );
    wifi_controller.set_config(&client_config).expect("Failed to set WiFi config");

    // WiFi start
    wifi_controller.start().expect("Failed to start WiFi");
    println!("WiFi started: {:?}", wifi_controller.is_started());

    // WiFi: scan for networks.
    // NOTE: scan_with_config() is a blocking scan. It will return the list of found networks.
    println!("Starting Wifi scan...");
    let scan_config = ScanConfig::default()
        // .with_ssid(SSID).  // only scan for 1 SSID
        .with_max(10);        // max networks to return
    let res = wifi_controller.scan_with_config(scan_config).expect("Wifi scan failed");
    for ap in res {
        println!("Found WiFi: {:?}", ap);
    }
    println!("WiFi capabilities: {:?}", wifi_controller.capabilities().unwrap());

    // Connect to WiFi
    // NOTE this method is non-blocking. Loop over is_connected() and wait.
    wifi_controller.connect().expect("Connect failed");
    loop {  // wait until connected
        match wifi_controller.is_connected() {
            Ok(true) => break,
            Ok(false) => {}
            Err(err) => {
                panic!("WiFi error: {:?}", err);
            }
        }
    }
    println!("WiFi connected: {:?}", wifi_controller.is_connected().expect("WiFi failed to connect"));



    // === TCP/IP
    // Init smoltcp: A TCP/IP stack designed for bare-metal, real-time systems without a heap.
    // STA: Statin-Mode WiFi devices (i.e. not AP)
    let mut device = interfaces.sta;
    // Current time
    let timestamp = smoltcp::time::Instant::from_micros(
        esp_hal::time::Instant::now().duration_since_epoch().as_micros() as i64,
    );
    // smoltcp interface
    let iface = smoltcp::iface::Interface::new(
        // Configure it with the actual device's MAC address
        smoltcp::iface::Config::new(smoltcp::wire::HardwareAddress::Ethernet(
            smoltcp::wire::EthernetAddress::from_bytes(&device.mac_address()),
        )),
        &mut device,
        timestamp,
    );

    // DHCP: Prepare a DHCP client (socket)
    // Socket set: space for storing sockets. N=3. Static allocation at compile time.
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let mut socket_set = SocketSet::new(&mut socket_set_entries[..]);
    // Init a DHCP socket and move it into the socket set.
    socket_set.add({
        let mut dhcp_socket = smoltcp::socket::dhcpv4::Socket::new();
        dhcp_socket.set_outgoing_options(&[DhcpOption{
            // DHCP Option 12: hostname
            kind: 12,
            data: b"esp-radio",
        }]);
        dhcp_socket
    });
    // Wait for getting an ip address
    let rng = Rng::new();
    let now = || Instant::now().duration_since_epoch().as_millis();

    // Create a WiFi stack using smoltcp iface, WiFi device, and socket set.
    // - random() is needed to pick the local port for the DHCP client.
    // - now: fn to get the current time, ms since epoch. It has to be monotonic, I guess.
    let stack = Stack::new(iface, device, socket_set, now, rng.random());

    // DHCP: get an IP address
    println!("Getting IP address...");
    loop {
        stack.work();
        if stack.is_iface_up() {
            println!("DHCP got IP address: {:?}", stack.get_ip_info().expect("DHCP failed to get an IP"));
            break;
        }
    }




    //=== Connected
    // Now we are connected.
    println!("Connected");

    // Prepare a socket to be used for HTTP requests.
    // It needs two buffers.
    let mut rx_buffer = [0u8; 1536];
    let mut tx_buffer = [0u8; 1536];
    let mut socket = stack.get_socket(&mut rx_buffer, &mut tx_buffer);

    loop {
        // Let the TCP stack make progress.
        // Pumps the network stack - processes packets, handles retransmissions, updates TCP state machines.
        // Make sure to call this function regularly.
        // It delegates to WifiStack::work()
        socket.work();

        // Open a TCP socket.
        // Use a pre-known IP address of the server.
        socket
            .open(IpAddress::Ipv4(Ipv4Addr::new(142, 250, 185, 115)), 80)
            .unwrap();
        // Send some HTTP bytes
        socket
            .write(b"GET / HTTP/1.0\r\nHost: www.mobile-j.de\r\n\r\n")
            .unwrap();
        // Actually sends the data.
        // write() was non-blocking: it only wrote to the buffer.
        socket.flush().unwrap();

        // Keep reading from the socket; timeout after 20 seconds.
        let deadline = time::Instant::now() + Duration::from_secs(20);
        let mut buffer = [0u8; 512];
        while let Ok(len) = socket.read(&mut buffer) {
            let received = unsafe { core::str::from_utf8_unchecked(&buffer[..len]) };
            print!("Recv: {}", received);

            if time::Instant::now() > deadline {
                println!("Timeout");
                break;
            }
        }

        // Done with the socket
        socket.disconnect();

        // For 5 more seconds, let the stack do its work.
        let deadline = time::Instant::now() + Duration::from_secs(5);
        while time::Instant::now() < deadline {
            socket.work();
        }
    }

}
