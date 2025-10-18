#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![feature(impl_trait_in_assoc_type)]
esp_bootloader_esp_idf::esp_app_desc!();
use {esp_backtrace as _, esp_println as _};
extern crate alloc;

use defmt;
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    interrupt::software::SoftwareInterruptControl,
    gpio,
};

// $ cargo add embassy-executor --features task-arena-size-65536
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

// Our library
use b08_wifi_http_client::wifi;
use b09_wifi_http_server::{webserver, led};

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 64 * 1024);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // RTOS
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    // Init WiFi & network stack
    let stack = wifi::start_wifi(&spawner, peripherals.WIFI).await;

    // Spawn webserver tasks
    let web_app = webserver::WebApp::default();
    for id in 0..webserver::WEB_TASK_POOL_SIZE {
        spawner.must_spawn(webserver::web_task(id, stack, web_app.router, web_app.config));
    }
    defmt::info!("Web server started!");

    // Spawm LED tasks
    spawner.must_spawn(led::led_task(
        gpio::Output::new(peripherals.GPIO8, gpio::Level::Low, gpio::OutputConfig::default()),
    ));

    // Sleep
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
