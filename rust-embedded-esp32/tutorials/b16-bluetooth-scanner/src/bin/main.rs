#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
esp_bootloader_esp_idf::esp_app_desc!();
use {esp_backtrace as _, esp_println as _};
extern crate alloc;

use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    gpio,
};
use embassy_executor::Spawner;
use embassy_time::Timer;

// Bluetooth
use trouble_host::prelude::*;
use esp_radio::{
    ble::controller::BleConnector,
};

// Util
use static_cell::StaticCell;

// Our module
use b16_bluetooth_scanner::blscanner;


// BLE scanner with a buzzer.
// Produces more clicks as you approach a specific BLE device identified by MAC address.
// NOTE: BLE devices randomize their addresses! You can't rely on a MAC address. You'll need to pair.
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // RTOS
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    // GPIO Active Buzzer.
    // It's open-drain and active-LOW.
    let buzzer_pin = peripherals.GPIO4;
    let mut buzzer = gpio::Output::new(buzzer_pin, gpio::Level::High, gpio::OutputConfig::default().with_drive_mode(gpio::DriveMode::OpenDrain));

    // ESP Bluetooth. Radio. Transport.
    let radio = {
        static RADIO: StaticCell<esp_radio::Controller> = StaticCell::new();
        RADIO.init(esp_radio::init().expect("Init radio"))
    };
    let bluetooth = BleConnector::new(radio, peripherals.BT, Default::default()).expect("Init bluetooth");

    // Run BT tasks
    blscanner::run_tasks(&spawner, bluetooth, buzzer).await.expect("Run bluetooth");

    // Idle run
    loop {
        Timer::after_secs(1).await;
    }
}
