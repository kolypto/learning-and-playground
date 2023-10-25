// Don't use the standard library
#![no_std]
// Don't use main(), which is tailored for command-line applications.
// Instead, we'll use the #[entry] attribute from the `xtensa-lx-rt` crate
#![no_main]

// Installs a panic handler.
// You can use other crates, but `esp-backtrace` prints the address which can be decoded to file/line
use esp_backtrace as _;
// Provides a `println!` implementation
use esp_println::println;
// Bring some types from `esp-hal`
use hal::{peripherals::Peripherals, prelude::*};
use hal::{clock::ClockControl, Delay, timer::TimerGroup, Rtc};
use hal::{IO};

// The entry point.
// It must be a "diverging function": i.e. have the `!` return type.
#[entry]
fn main() -> ! {
    // HAL drivers usually take ownership of peripherals accessed via the PAC
    // Here we take all the peripherals from the PAC to pass them to the HAL drivers later
    let peripherals = Peripherals::take();
    // Sometimes a peripheral is coarse-grained and doesn't exactly fit the HAL drivers.
    // Here we split the SYSTEM peripheral into smaller pieces which get passed to drivers
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    // Disable the RTC.
    // Instantiate it, and disable.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    rtc.rwdt.disable();

    // Disable TIMG watchdog timers: if not, the SoC would reboot after some time.
    // Another way is to feed the watchdog.
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks, &mut system.peripheral_clock_control);
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks, &mut system.peripheral_clock_control);
    let mut wdt0 = timer_group0.wdt;
    let mut wdt1 = timer_group1.wdt;
    wdt0.disable();
    wdt1.disable();

    // Set GPIO2 as an output, and set its state high initially.
    // GPIO2 = LED
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio2.into_push_pull_output();
    led.set_high().unwrap();

    // Set GPIO36 as input
    let button = io.pins.gpio36.into_pull_up_input();

    // Init a timer
    let mut delay = Delay::new(&clocks);
    loop {
        println!("Loop...");
        // Blink
        // led.toggle().unwrap();

        if button.is_high().unwrap() {
            for _ in 0..10 {
                led.set_high().unwrap();
                delay.delay_ms(50u32);
                led.set_low().unwrap();
                delay.delay_ms(50u32);
            }
        }

        delay.delay_ms(500u32);
    }
}
