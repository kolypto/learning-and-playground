#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
esp_bootloader_esp_idf::esp_app_desc!();


use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    gpio,
    main,
};
use log::info;

// Embassy
// $ cargo add esp-hal --features unstable   # requires unstable features
// $ cargo add esp-rtos --features esp32c3,embassy,log-04
// $ cargo add embassy-executor embassy-time
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
#[cfg(target_arch = "riscv32")]
use esp_hal::interrupt::software::SoftwareInterruptControl;


#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // Init a timer group. Embassy will use it.
    #[cfg(target_arch = "riscv32")]
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    // Start Embassy RTOS
    esp_rtos::start(
        timg0.timer0,
        #[cfg(target_arch = "riscv32")]
        sw_int.software_interrupt0,
    );

    // LED
    let led = gpio::Output::new(peripherals.GPIO8, gpio::Level::High, gpio::OutputConfig::default());

    // Start a task. Pass the LED to it.
    spawner.spawn(blink_led(led)).ok();

    // Sleep properly.
    loop {
        Timer::after(Duration::from_millis(5_000)).await;
    }
}

// An async task
#[embassy_executor::task]
async fn blink_led(mut led: gpio::Output<'static>) {
    loop {
        led.toggle();

        // Give up
        Timer::after(Duration::from_millis(300)).await;
    }
}
