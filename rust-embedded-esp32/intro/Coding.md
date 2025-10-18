## New Project

### `no_std`

```console
$ esp-generate -c esp32c3
```

Examples:

```console
$ esp-generate -c esp32c3 -o stack-smashing-protection -o esp-backtrace -o log -o unstable-hal -o wokwi
$ esp-generate -c esp32c3 -o defmt -o unstable-hal -o embassy
```

### `std` IDF

```console
$ cargo generate esp-rs/esp-idf-template cargo
```

Add this to `.cargo/config.toml` to

```toml
[env]
# share IDF across projects (3Gb!)
ESP_IDF_TOOLS_INSTALL_DIR = { value = "global" }
# Faster espflash
ESPFLASH_BAUD=921600
```


## Abstraction Layers

Each abstraction layer offers a different balance between flexibility and ease of use:

1. Peripheral Access Crate (PAC): auto-generated crates that provide type-safe access to a microcontroller's peripherals.
   Typically generated from the manufacturer's SVD files (`svd2rust`).
   PACs give you a structured and safe way to interact directly with hardware registers.
2. Hardware Abstraction Layer (HAL): builds on top of PAC to provide higher-level interfaces to the microcontroller's peripherals:
   GPIO, SPI, I2C, UART. HALs usually implement traits from `embedded-hal` which are standard and platform-independent.
   Crate `esp-hal` implements traits from `embedded-hal`.
3. Board Support Package (BSP): tailored to specific development boards.
   Provides interfaces to on-board LEDs, buttons, sensors, etc.

There are even lower levels below PACs:

1. Micro architecture crate: crates that are specific to the processor core architecture.
   They enable/disable interrupts, access internal timers, ...
   For RISC-V ESP32s, these are `cortex-m` and `cortex-m-rt`.
2. Raw MMIO (Memory-Mapped IO): means directly working with hardware registers by reading and writing to specific memory addresses.
   This operation would be `unsafe` in Rust.

## `no_std` App Skeleton

Disable std and declare `main()`:

```rust
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


use esp_hal::{ main };

// This function will be called by the reset handler after RAM is initialized.
// NOTE: it must never return: it should be running forever!
#[main]
fn main() -> ! {
   loop {}
}
```

### Panic Handler

Rust needs a panic handler. `std` used to provide it -- but now we're on our own.
We'll need to define a panic handler: a function called when Rust panics.

```rust
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    // This function must never return
    loop {}
}
```

This simple function is already implemented in the `panic_halt` crate:

```rust
// the program just stops and stays in an infinite loop.
use panic_halt as _;
```

Instead, we use `esp_backtrace` that provides a panic handler that prints a backtrace:

```rust
use esp_backtrace as _;  // Provides a panic handler
```

Note that `esp-backtrace` has issues with defmt.
One solution is to use `--feature println` instead.

Use a custom handler if you want defmt:

```rust

// We'll use a custom panic handler.
// Because `esp_backtrace` doesn't print the panic message (!)
#[panic_handler]
fn panic(p: &core::panic::PanicInfo) -> ! {
    // Print the panic message
    if let Some(loc) = p.location() {
        defmt::error!(
            "PANIC at {}:{}:{}",
            loc.file(),
            loc.line(),
            loc.column()
        );
    }

    defmt::error!("PANIC: {}", defmt::Display2Format(&p.message()));

    // This function must never return
    loop {}
}

```

### Logging

In Rust, two prominent logging frameworks are commonly used:
* `defmt` is a highly efficient logging framework designed for resource-constrained environment.
  It offers compact, binary-encoded log messages, reducing the overhead associated with traditional string-based logging.
* `log` is a widely adopted logging facade in the Rust community.

To use `log`:

```rust
use log;  // Use for output/logging: "log" or "defmt"

fn main() -> ! {
   // Binds `log` to `esp_println`: so that `log` knows how to print strings (to UART)
   esp_println::logger::init_logger_from_env();

   // Now you can print
   log::info!("Board starting...");
}
```

To use `defmt`:

```rust
use defmt;

fn main() -> ! {
    // TODO: doesn't compile. Why?
    defmt::trace!("trace");
    defmt::debug!("debug");
    defmt::info!("info");
    defmt::warn!("warn");
    defmt::error!("error");
}
```

>  `esp-println` has a `defmt-espflash` feature, which adds framming bytes so `espflash` knows that is a defmt message.
> `esp-backtrace` has a defmt feature that uses defmt logging to print panic and exception handler messages.
>
> NOTE: `espflash` requires framming bytes as when using defmt it also needs to print non-defmt messages
> (like the bootloader prints). It's important to note that other defmt-enabled tools like probe-rs
> won't be able to parse these messages due to the extra framing bytes.

Another option: `esp_println`, but actually, `log` uses it under the hood:

```rust
use esp_println::println;

fn main() -> ! {
   println!("Hi there! The chip is starting...");
}
```

### Periperals

In embedded Rust, peripherals are singletons: this ensures that there is never any conflict over controlling the hardware.

We won't be calling `Peripherals::take()` directly. Instead, we will use the `esp_hal::init(config)` function:
it calls `Peripherals::take()` internally, and also does some basic system setup: stack guard, cpu clock, rtc clock, interrupt handlers, disable watchdog timers.

```rust
fn main() -> ! {
    let peripherals = esp_hal::init(
      // sets the CPU clock to its maximum frequency.
      // Slower clock => less power consumption and heat.
      // When to slow down: battery-powered devices, simple tasks, sleeping most of the time
      // When to max out: don't care about power, WiFi/BLE active (they need CPU speed), realtime requirements
      esp_hal::Config::default()
         .with_cpu_clock(CpuClock::max())
   );
}
```

## Sleeping

Busy wait: waste CPU cycles doing nothing. Alright for short periods:

```rust
use esp_hal::{
    time::{Duration, Instant},
};

// Blocking delay, busy wait: burns CPU cycles until `duration` passes.
fn busy_wait(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
```

Actually, this function is already implemented in `esp_hal::delay`.
It's still busy wait:

```rust
use esp_hal::delay::Delay;

let delay = Delay::new();
delay.delay_micros(2); // busy wait
```


## Time

RTC is the Real-Time Clock: a peripheral that handles actual calendar time (year, month, day, hour, minute, second) —
if initiated with NTP.
Survives deep sleep if RTC is powered.
Is slower to read.

```rust
use esp_hal::{
   // Real-Time control and Low Power management.
   rtc_cntl::Rtc,
};

// init the peripheral
let rtc = Rtc::new(peripherals.LPWR);

// current RTC time in milliseconds
let time_ms = rtc.current_time_us() / 1000;
```

Another approach: `Instant::now()` returns the number of ticks elapsed.
No ticks while asleep. Resets on every boot/wake. Fast to read.

Use for relative timing:

```rust
// Instant: "500ms have passed since I started"
let start = Instant::now();
do_work();
let elapsed = start.elapsed();
```


## Linker

You don't have to do this manually: `esp-generate` produces a valid template.
Have a look at this line:

```rust
// build.rs

fn main() {
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}
```

or:

```toml
# .cargo/config.toml
[target.riscv32imc-unknown-none-elf]
runner = "espflash flash --monitor --chip esp32c3"
rustflags = ["-C", "link-arg=-Tlinkall.x"]
```

With embedded systems, we must tell the linker exactly where code, data, and stack should go in the memory.
The linker script is a text file that tells the linker: where to place code in memory, where the stack starts,
and how big each memory region is.

The linker script we will use is called `linkall.x`: it's provided by `esp-hal`.
Have a look at its source: it maps memory regions: [linkall.x](https://github.com/esp-rs/esp-hal/tree/main/esp-hal/ld/esp32c3)




## Allocation

In a `no_std` environment, the [`alloc`](https://doc.rust-lang.org/alloc/) crate is available as an option for heap allocation.
This enables useful common Rust items such as `Vec` and `Box` and other collections that require heap allocation.
In the some cases, `alloc` may be required for a dependency you wish to use.

Espressif provides their own `no_std` heap allocator, [`esp-alloc`](https://crates.io/crates/esp-alloc).

Why not use a heap? Allocations lead to memory fragmentation and have runtime overhead.
What's the problem?
Because some Espressif chips have non-contiguous memory mapping: not all physical RAM is usable as a single, flat heap.

For example, some regions are reserved for ROM code usage, and cannot be overwritten.
In ESP32, there are three SRAM regions: SRAM0 (128Kb), SRAM1 (128Kb) and SRAM2 (200Kb),
where SRAM1 and SRAM2 have reserved regions.
So you see, there's no single continuous space.

There is also some memory that the 2nd stage bootloader uses during the boot process that can't be used as stack, but could be used as heap instead once in the main application. You can use the #[ram(reclaimed)] macro in the heap allocator declaration to use this otherwise unused memory.


```rust
// Use 64kB in dram2_seg for the heap, which is otherwise unused.
heap_allocator!(#[ram(reclaimed)] size: 64000);
```

Our chips have a few hundred kilobytes of internal RAM, which could be insufficient for some applications.
Some Espressif chips have the ability to use virtual addresses for external PSRAM (Pseudostatic RAM) memory:
it is usable in the same way as internal data RAM, with certain restrictions:
see [External RAM Restrictions](https://docs.espressif.com/projects/esp-idf/en/v5.4.1/esp32/api-guides/external-ram.html#restrictions)




## Configuration

One way is to use `env!()` to load env variables *at compile time* and embed them into the code:

```rust
// Read constants from env at compile time (!)
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");
```

Also see [toml-cfg](https://github.com/jamesmunns/toml-cfg):
put your settings into `cfg.toml`:

```toml
[your-package]  # your package name
user = "example"
password = "h4ckm3"
```

then get them into Rust struct:

```rust
#[derive(Debug)]
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    user: &'static str,
    #[default("")]
    password: &'static str,
}
```

















## Async

Drivers from `esp-hal` are [`Blocking`](https://docs.espressif.com/projects/rust/esp-hal/1.0.0-rc.0/esp32c6/esp_hal/struct.Blocking.html) by default.
Convert them to [`Async`](https://docs.espressif.com/projects/rust/esp-hal/1.0.0-rc.0/esp32c6/esp_hal/struct.Async.html) mode by calling `.into_async()`.

ESP32 supports [Embassy](https://embassy.dev/) through the [`esp-rtos`](https://crates.io/crates/esp-rtos) crate:
it provides integration between esp-hal and the Embassy asynchronous framework.

Also check:

* [ArielOS](https://github.com/ariel-os/ariel-os). It is more powerful and has integrations with Embassy.
* [RTIC](https://crates.io/crates/rtic) is a community supported concurrency framework for real-time systems.
  Currently, only ESP32-C3 and ESP32-C6 are supported.


## Testing

Test as much as possible on your host machine: it's faster, easier, and won't waste flash write cycles on your device.

For Hardware In Loop (HIL) testing, use [`embedded-test`](https://github.com/probe-rs/embedded-test) framework.
We use `probe-rs` to flash and run tests on the target device. To do this, you must use only the USB-Serial-JTAG port on your DevKit
If your device does not have such a port, you will have to use [`esp-prog`](https://docs.espressif.com/projects/esp-dev-kits/en/latest/other/esp-prog/user_guide.html)
or another suitable programmer and connect it according to the [connection instructions](https://docs.espressif.com/projects/esp-idf/en/v5.2.3/esp32s2/api-guides/jtag-debugging/configure-other-jtag.html)
(select the desired chip on the page).



## Over-the-Air Updates (OTA)

OTA is heavily reliant on a bootloader to handle the switching, replacement and rollback of OTA images (firmware updates).
See the small OTA example in the esp-hal repository: <https://github.com/esp-rs/esp-hal/tree/main/examples/ota>



## Compilation and Binary Size

Advices:
* Embedded Rust Book: [Optimizations: the speed size tradeoff](https://docs.rust-embedded.org/book/unsorted/speed-vs-size.html) of The Embedded Rust Book.
* Minimizing Rust Binary Size: [min-sized rust](https://github.com/johnthagen/min-sized-rust)
* Embassy FAQ: [Why is my binary so big?](https://embassy.dev/book/#_why_is_my_binary_so_big)
* Embassy FAQ: [How can I measure resource usage (CPU, RAM, etc.)?](https://embassy.dev/book/#_how_can_i_measure_resource_usage_cpu_ram_etc)



## Libraries

### Anyhow

Provides anyhow::Error for easy error handling:

```rust
anyhow::Result<T> //= anyhow::Result<T, Error>
bail!("Missing attribute: {}", missing); //= return Err(anyhow!(..));
ensure!(user == 0, "only user 0 is allowed");  // assertion
```

Install without std:

```console
$ cargo add anyhow --no-default-features
```

### Heapless

Statically allocated objects: strings, etc.

