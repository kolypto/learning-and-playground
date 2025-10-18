#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![feature(impl_trait_in_assoc_type)]
use {esp_backtrace as _, esp_println as _};
esp_bootloader_esp_idf::esp_app_desc!();

// Memory allocations
// Use this create instead for heap allocations: Box, Rc, RefCell, Arc.
// esp-radio needs it.
extern crate alloc;

use defmt;
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    rng::Rng,
    interrupt::software::SoftwareInterruptControl,
};

// Embassy
// $ cargo add esp-hal --features unstable   # requires unstable features
// $ cargo add esp-rtos --features esp32c3,embassy,log-04
// $ cargo add embassy-executor --features nightly
// $ cargo add embassy-time
// $ cargo add embassy-net --features defmt,tcp,udp,dhcpv4,dhcpv4-hostname,medium-ethernet,dns
// $ cargo add smoltcp --features dns-max-server-count-4
// $ cargo add reqwless --features embedded-tls,defmt
use esp_rtos;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_net::{dns::DnsSocket, tcp::client::{TcpClient, TcpClientState}};

// Reqwless: HTTP client
use reqwless::client::{HttpClient, TlsConfig};

// Our library
use b08_wifi_http_client as lib;


#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // Init allocator: 64K in reclaimed memory + 66K in default RAM
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 64 * 1024);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    // CPU Clock: WiFi in ESP32 requires a fast CPU
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // Init Embassy the usual way
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    // Init WiFi & network stack
    let stack = lib::wifi::start_wifi(&spawner, peripherals.WIFI).await;
    let rng = Rng::new();
    let tls_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    // Load the website
    load_url(stack, tls_seed).await;

    // Go
    loop {
        Timer::after(Duration::from_millis(500)).await;
    }
}

// HTTP request using the network stack and reqwless.
// HTTP Request.
// First, initialize the DNS socket and the TCP client.
// Now we init TLS configuration using a random number.
async fn load_url(stack: embassy_net::Stack<'_>, tls_seed: u64) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let dns = DnsSocket::new(stack);
    let tcp_state = TcpClientState::<1, 4096, 4096>::new();
    let tcp = TcpClient::new(stack, &tcp_state);

    let tls = TlsConfig::new(
        tls_seed,
        &mut rx_buffer,
        &mut tx_buffer,
        reqwless::client::TlsVerify::None,
    );

    let mut client = HttpClient::new_with_tls(&tcp, &dns, tls);
    let mut buffer = [0u8; 4096];
    let mut http_req = client
        .request(
            reqwless::request::Method::GET,
            "https://jsonplaceholder.typicode.com/posts/1",
        )
        .await
        .unwrap();
    let response = http_req.send(&mut buffer).await.unwrap();

    defmt::info!("Got response");
    let res = response.body().read_to_end().await.unwrap();

    let content = core::str::from_utf8(res).unwrap();
    defmt::println!("{}", content);
}
