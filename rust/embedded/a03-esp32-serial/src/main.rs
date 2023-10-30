#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    prelude::{*, nb::block},
    peripherals::Peripherals,
    peripherals::Interrupt,
    clock::ClockControl, Delay,
    IO, Uart,
    interrupt, uart,
};
use esp_backtrace as _;
// use debouncr::{debounce_3, Edge};  // debounce button press
use core::fmt::Write;  // writeln!() here
use embedded_io;  // read(&buf)

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
            baudrate: 115_200,  // the speed
            ..Default::default()
        }),
        // The pings to use: Tx and Rx
        Some(hal::uart::TxRxPins::new_tx_rx(
            // our board has a CP2102 USB-UART converter connected to U0TXD (GPIO1)  and U0RXD (GPIO3) pins.
            // Let's use them: then all our output to this `uart0` will end up in /dev/ttyUSB0 :)
            io.pins.gpio1.into_push_pull_output(),
            io.pins.gpio3.into_floating_input(),
        )),
        &clocks,
        &mut system.peripheral_clock_control,
    );
    // .. or do the same thing, with defaults:
    // let mut uart0 = Uart::new(peripherals.UART0, &mut system.peripheral_clock_control); // with defaults


    // Example: write to UART while the button is pressed
    if false {
        loop {
            // Toggle the LED while the button is pressed
            if !button.is_input_high() {
                led.toggle().unwrap();

                // Speak into UART
                writeln!(uart0, "Button Press!\n").unwrap();
            }

            // Sleep
            delay.delay_ms(50u32);
        }
    }

    // Example: echo server. Reading from UART
    // Heapless: static data structures that don't require dynamic memory allocation
    // All heapless data structures store their memory allocation inline:
    use heapless::Vec;

    // Allocate a buffer: can store up to 128 bytes
    let mut buf: Vec<u8, 128> = Vec::new();

    loop {
        buf.clear();

        // TODO: how to read the whole string?
        // let n =embedded_io::Read::read(&mut uart0, &mut buf).unwrap();

        loop {
            // Read bytes into the buffer
            let byte = nb::block!(uart0.read()).unwrap();
            if buf.push(byte).is_err() {
                write!(uart0, "error: buffer full\n").unwrap();
                break;
            }

            // Enter. Done.
            if byte == 13 {  // <enter>
                // Reverse the string and print it
                for byte in buf.iter().rev().chain(&[b'\n']) {
                    nb::block!(uart0.write(*byte)).unwrap();
                }
                break;
            }
        }

        nb::block!(uart0.flush()).unwrap()
    }
}
