// The #![no_std] attribute disables the use of the standard library (std):
// because we don't have an OS to allocate the memory for us.
#![no_std]
// The #![no_main] disables the std main(): we'll bring our own entrypoint.
// Decorate your `fn main()` with #[main]
#![no_main]
// Prevents accidentally calling mem::forget() on ESP HAL types, which would cause memory leaks or hardware lockups.
// Problem: ESP HAL types often hold DMA buffers or manage ongoing hardware operations.
// Solution: #![deny()] makes it a compile error if you try to use mem::forget()
#![deny(clippy::mem_forget)]
// Embeds metadata into your binary that the ESP-IDF bootloader expects to find:
// a default app-descriptor, with app version and other metadata.
// The second-stage bootloader (from ESP-IDF) validates the app image before running it by checking this descriptor.
esp_bootloader_esp_idf::esp_app_desc!();


use esp_backtrace as _;  // Provides a panic handler
use log;  // Use for output/logging: "log" or "defmt"

use esp_hal::{
    clock::CpuClock, gpio, ledc::{self, channel::ChannelHW}, main, time::{Duration, Instant, Rate}
};

// Node that in no_std main() can't have a return value.
#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // Init a LED: the onboard LED.
    // Initial state = HIGH
    let mut onboard_led = gpio::Output::new(peripherals.GPIO8, gpio::Level::High, gpio::OutputConfig::default());

    // Blink it
    log::info!("Blink happily");
    blink_led(&mut onboard_led, 3);

    // Init PWM.
    // Note that in `esp-hal` this is an unstable feature:
    // $ cargo add esp-hal unstable
    // Currently only supports fixed-frequency output.
    let mut ledc = ledc::Ledc::new(peripherals.LEDC);
    // Set global slow clock source. (Note: high-speed PWM is not available on ESP32-C3/C6)
    ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk);

    // Get a new timer
    let mut lstimer0 = ledc.timer::<ledc::LowSpeed>(ledc::timer::Number::Timer0);
    use ledc::timer::TimerIFace;  // Bring in: .configure()
    lstimer0.configure(ledc::timer::config::Config {
        clock_source: ledc::timer::LSClockSource::APBClk,
        // We'll set the frequency to 24 kHz.
        // > duty_resolution_bits = log2( clock_freq / ( desired_pwm_freq * clk_div ) ), where
        // Solve for `clk_div`=1 (max) and `clk_div`=1023 (min)
        // For 24 Khz: min=2, max=12. Choose any number in between.
        frequency: Rate::from_khz(24),
        duty: ledc::timer::config::Duty::Duty5Bit,
    }).unwrap();

    // PWM channel. Configure.
    // It maps a timer to a GPIO pin.
    use ledc::channel::ChannelIFace;  // Bring in: .configure()
    let mut channel0 = ledc.channel(ledc::channel::Number::Channel0, onboard_led);
    channel0.configure(ledc::channel::config::Config {
        timer: &lstimer0,
        // Duty percentage.
        // 10% => 90% brightness
        // 90% => 10% brightness
        duty_pct: 90,
        // How to drive the pin
        drive_mode: gpio::DriveMode::PushPull,
    }).unwrap();

    channel0.set_duty(30); // 30%

    loop {
        // PWM has `start_duty_fade()`: gradually changes from one duty cycle percentage to another.
        // Fade in
        channel0.start_duty_fade(30, 100, 500).unwrap();
        while channel0.is_duty_fade_running() {} // wait
        // Fade out
        channel0.start_duty_fade(100, 30, 500).unwrap();
        while channel0.is_duty_fade_running() {} // wait

        // Sleep. Busy wait:
        busy_wait(Duration::from_millis(500));
    }
}

// Blocking delay: burns CPU cycles until `duration` passes.
fn busy_wait(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}

// Blink LED: short-looong
fn blink_led(led: &mut gpio::Output, n: u8) {
    for _ in 0..n {
        led.toggle();
        busy_wait(Duration::from_millis(50));
        led.toggle();
        busy_wait(Duration::from_millis(200));
    }
}