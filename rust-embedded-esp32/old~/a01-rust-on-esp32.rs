// Don't use the standard library.
// Don't use main(), which is tailored for command-line applications.
// Instead, we'll use the #[entry] attribute from the `xtensa-lx-rt` crate
#![no_std]
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
    // Here we take all the peripherals from the PAC to pass them to the HAL drivers later.
    // This is singleton design pattern: one owner, here:
    let peripherals = Peripherals::take();

    // Sometimes a peripheral is coarse-grained and doesn't exactly fit the HAL drivers.
    // Here we split the SYSTEM peripheral into smaller pieces which get passed to drivers.
    // `split()` creates a `hal::system::SystemParts` HAL structure from a PAC structure:
    // we are sort of "splitting" a PAC structure into HAL structures: peripherals, registers, etc.
    let mut system: hal::system::SystemParts = peripherals.DPORT.split();

    // Configure the CPU clock: use the max() frequency. Then apply (freeze)
    // let clocks = ClockControl::boot_defaults(system.clock_control).freeze(); // defaults
    // let clocks = ClockControl::max(system.clock_control).freeze(); // max speed
    let clocks = match "max" {
        // Choose a frequency
        "max" => ClockControl::max,
        _ => ClockControl::boot_defaults,
    }(system.clock_control).freeze();



    // Disable watchdogs.
    // The ESP32 chip has three watchdog timers "wdt": one in each of the two timer modules, and one in the RTC module.
    // Only the RDWT (RTC Watchdog) can trigger the "system reset" (reset the entire chip)
    // The two other timers can do: "interrupt", "CPU reset", "core reset".
    // They have four stages each, each with a timeout and action.
    // Let's disable them all.

    // Disable the RWDT (RTC watchdog).
    // "RTC" is a real-time clock. It can wake the device up from a low power mode.
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    rtc.rwdt.disable();

    // Disable TIMG watchdog timers ("MWDT": main watchdog timers).
    // If not, the SoC would reboot after some time.
    // Another way is to feed the watchdog.
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks, &mut system.peripheral_clock_control);
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks, &mut system.peripheral_clock_control);
    let mut wdt0 = timer_group0.wdt;
    let mut wdt1 = timer_group1.wdt;
    wdt0.disable();
    wdt1.disable();


    // App config file.
    // Values are read from "cfg.toml" during build.
    let app_config = CONFIG;



    // IO handle: it enables us to create handles for individual pins.
    // GPIOs need to be configured before use:
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);



    // Set GPIO2 as an output: this is the LED.
    // Configure it to be a PUSH-PULL output: i.e. low=GND (push down), high=VCC (pull up).
    // Set its initial state: HIGH
    let mut led = io.pins.gpio2.into_push_pull_output();
    led.set_high().unwrap();

    // Set GPIO36 as input
    let button = io.pins.gpio36.into_pull_up_input();



    // Init a timer
    // The MCU has several timers. They can do things for us: e.g. pause the execution for some time.
    let mut timer0 = timer_group0.timer0;

    // Or use this lame `Delay` which is a busy-wait timer: loop + count cycles
    let mut delay = Delay::new(&clocks);

    loop {
        println!("Loop...");
        // Blink
        // led.toggle().unwrap();

        // If the button is pressed (or the pin is touched) — blink
        if button.is_high().unwrap() {
            for _ in 0..10 {
                led.set_high().unwrap();
                delay.delay_ms(50_u32);

                led.set_low().unwrap();
                delay.delay_ms(50_u32);
            }
        }

        // Sleep a bit
        delay.delay_ms(app_config.loop_timer);

        // Sleep using a timer
        let mut del_var = app_config.loop_timer.millis();
        timer0.start(del_var);
        while timer0.wait() != Ok(()) {}
    }
}


/// This configuration is picked up at compile time from `cfg.toml`.
#[toml_cfg::toml_config]
pub struct Config {
    //#[default("")]
    //wifi_ssid: &'static str,

    #[default(500u32)]
    loop_timer: u32,
}
