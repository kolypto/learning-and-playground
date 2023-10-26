#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    prelude::*,
    peripherals::Peripherals,
    peripherals::Interrupt,
    clock::ClockControl, Delay,
    IO, Uart,
    interrupt,
};
use esp_backtrace as _;
// use debouncr::{debounce_3, Edge};  // debounce button press
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system: hal::system::SystemParts = peripherals.DPORT.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let mut delay = Delay::new(&clocks);

    // Logging
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");

    // Get hold of the LED and the button
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio2.into_push_pull_output();
    let mut button = io.pins.gpio0.into_pull_up_input();

    // Get hold of UART.
    // With ESP32, it can be mapped to any GPIO pins, but we use TX/RX pins connected to USB:
    let mut uart0 = Uart::new_with_config(
        peripherals.UART0,
        Some(hal::uart::config::Config{
            baudrate: 115_200,
            ..Default::default()
        }),
        Some(hal::uart::TxRxPins::new_tx_rx(
            // our board has a CP2102 USB-UART converter connected to U0TXD (GPIO1)  and U0RXD (GPIO3) pins.
            // Let's use them: then all our output to this `uart0` will end up in /dev/ttyUSB0 :)
            io.pins.gpio1.into_push_pull_output(),
            io.pins.gpio3.into_floating_input(),
        )),
        &clocks,
        &mut system.peripheral_clock_control,
    );
    // let mut uart0 = Uart::new(peripherals.UART0, &mut system.peripheral_clock_control); // with defaults

    loop {
        // Toggle the LED while the button is pressed
        if !button.is_input_high() {
            led.toggle().unwrap();
            writeln!(uart0, "Button Press!\n").unwrap();
        }

        // Sleep
        delay.delay_ms(50u32);
    }
}
