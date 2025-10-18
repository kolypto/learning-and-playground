// We're running on bare metal: there's no OS, no heap allocator, no standard library.
#![no_std]
// #[main] from esp-hal replaces Rust's normal main because there's no OS to return to.
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]


// Provides a `println!` implementation
// Add me with:
// $ cargo add esp-println --features esp32c3
use esp_println::println;
use defmt;

// Installs a panic handler.
// You can use other crates, but `esp-backtrace` prints the address which can be decoded to file/line
// Add me with:
// $ cargo add esp-backtrace --features esp32c3,println
use esp_backtrace as _;

// Core primitives: RefCell, Mutex
use core::cell::RefCell;
use critical_section::Mutex;

// Bring some types from `esp-hal`
use esp_hal::{
    // HAL configuration
    clock::CpuClock,
    // Delay driver: this peripheral sleep()
    // NOTE: only available in esp-hal[unstable]
    delay::Delay,
    // GPIO control: configure, set low, set high
    gpio::{Output, OutputConfig, Level},
    gpio::{Input, InputConfig, Pull},
    // General Purpose Input/Output Driver
    gpio::{Io},
    // Interrupts
    gpio::{Event},
    handler,  // proc macro
    // Timing
    time::{Duration, Instant},
    // #[main] proc macro
    main,
};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();



// The entry point.
// It must be a "diverging function": i.e. have the `!` return type.
#[main]
fn main() -> ! {
    // Configure the hardware
    let config = esp_hal::Config::default()
        .with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);



    // === Example: Print to UART
    println!("Hi there! The chip is starting...");
    // panic!("This is a panic"); // the last thing you'd ever print

    //=== Example: print using defmt
    // defmt is a highly efficient logging framework.

    // `esp-println` has a `defmt-espflash` feature, which adds framming bytes so `espflash` knows that is a defmt message.
    // `esp-backtrace` has a defmt feature that uses defmt logging to print panic and exception handler messages.

    // NOTE: `espflash` requires framming bytes as when using defmt it also needs to print non-defmt messages
    // (like the bootloader prints). It's important to note that other defmt-enabled tools like probe-rs
    // won't be able to parse these messages due to the extra framing bytes.

    // TODO: doesn't compile. Why?
    defmt::trace!("trace");
    defmt::debug!("debug");
    defmt::info!("info");
    defmt::warn!("warn");
    defmt::error!("error");

    // === Example: busy wait is a bad way to sleep: it will burn CPU cycles
    let started_at = Instant::now();
    while started_at.elapsed() < Duration::from_millis(500) {}

    // === Example: Delay peripheral
    // Initialize the Delay driver
    // ❗ The name promises us a driver/peripheral, but in fact it's busy wait.
    //    It's fine for short waits. For longer waits, you should learn to wake up properly.
    let delay = Delay::new();
    delay.delay_millis(500);  // still busy wait


    // === Example: blinky using the `Delay` peripheral.
    println!("Blinking automatically...");

    // Configure GPIO7 as output. Set its state to HIGH initially.
    let mut led = Output::new(peripherals.GPIO7, Level::High, OutputConfig::default());
    led.set_high();

    loop {
        led.toggle();
        delay.delay_millis(250);

        // Blink for some seconds, then quit
        if started_at.elapsed() > Duration::from_secs(3) {
            break
        }
    }

    //=== Example: Click to blink
    // We will read the state of a button in a loop and light up the LED.
    println!("Blinking when you press...");

    // Most dev boards have a button.
    // We will use the BOOT button on GPIO9.
    // With pull-up: defaults to HIGH when nothing's connected; reads LOW when connected to ground.
    // Otherwise, when not grounded, will read random noise.
    let mut button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));

    loop {
        if button.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }

        // Blink for some seconds, then quit
        if started_at.elapsed() > Duration::from_secs(6) {
            break
        }
    }

    //=== Example: Detect button press with interrupt
    println!("Blinking when you press (interrupt)...");

    // Use `Io` go set an interrupt handler.
    // All GPIO pins share the same interrupt handler.
    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(handler);

    // Use a critical section to do things atomically.
    // We start listening on GPIO events; at the same time we provide the `Input` object to the handler.
    // `critical_section` Disables interrupts temporarily.
    // No interrupt can fire mid-execution.
    critical_section::with(|cs| {
        // Listen for interrupts.
        button.listen(Event::FallingEdge);

        // Use the static variable to pass the Input that has fired the handler.
        // Replace the actual value of the Option<Input>.
        BUTTON.borrow_ref_mut(cs).replace(button);

        // Also provide the led
        LED.borrow_ref_mut(cs).replace(led);
    });

    // Keep waiting
    loop {
    }

    // main() must not quit
    panic!("Done");
}

// Static button:
// we'll need to pass it from main() to the interrupt handler
// to clear the pending interrupt on the button
// - RefCell = runtime borrow checking.
// - Mutex = critical section wrapper for interrupt safety.
// Together they let you safely share mutable state between main() and interrupt handlers without data races.
static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));

// Static LED.
// The main() fn will pass it to us.
static LED: Mutex<RefCell<Option<Output>>> = Mutex::new(RefCell::new(None));

// Interrupt handler
// One interrupt handler to rule them all
#[handler]
fn handler() {
    // `critical_section` Disables interrupts temporarily.
    // No interrupt can fire mid-execution.
    critical_section::with(|cs| {
        println!("GPIO interrupt");

        // Do we have a button presset?
        // Get the `Input` passed to us through the mutable `Option<Input>`
        let mut button = BUTTON.borrow_ref_mut(cs);
        let Some(button) = button.as_mut() else {
            // Some other interrupt has occurred
            // before the button was set up.
            return;
        };

        if button.is_interrupt_set() {
            println!("Button pressed");

            // If you're listening to EventLow/EventHigh, events will keep firing.
            // Unlisten to stop the flow.
            // button.unlisten();
        }

        // LED toggle
        if let Some(led) = LED.borrow_ref_mut(cs).as_mut() {
            led.toggle();
        }

        // Clear the interrupt status bit for this pin.
        // Hardware sets an interrupt flag when the event occurs.
        // If you don't clear it, the interrupt fires again immediately in an infinite loop.
        button.clear_interrupt();
    });
}
