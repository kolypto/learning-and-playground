#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use {esp_backtrace as _, esp_println as _};
esp_bootloader_esp_idf::esp_app_desc!();
extern crate alloc;

use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

// Bluetooth
use trouble_host::prelude::*;
use esp_radio::{
    ble::controller::BleConnector,
};

// Our module
use b15_bluetooth::ble;

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // RTOS
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    // Init ESP Bluetooth. Radio. Transport.
    // The `BleConnector` is implements the portable abstraction `bt_hci::transport::Transport`:
    // we move the value to the module as soon as we've gotten from the ESP level to portable level.
    let radio_init = esp_radio::init().expect("Initialize radio");
    let bt_transport = BleConnector::new(&radio_init, peripherals.BT, Default::default()).unwrap();

    // Use our module
    ble::run(bt_transport).await.expect("Start BLE");


    // Idle run
    loop {
        Timer::after_secs(1).await;
    }
}
