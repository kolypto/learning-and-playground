# Rust on ESP32

Date: 2025-10






# intro
# Links

Rust-ESP32 books:

* ✅ [Rust on ESP Book](https://docs.espressif.com/projects/rust/book/) — hands-on introduction
* ✅ [impl Rust for ESP32](https://esp32.implrust.com/) — lots of practice ⭐
* [The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html) — an introduction to embedded development in general
* ✅ [Embedded Rust on ESP (no_std)](https://docs.espressif.com/projects/rust/no_std-training/) — examples with `no_std`; poor explanations 👎
* ✅ [Embedded Rust on ESP Training (std)](https://docs.esp-rs.org/std-training/) — examples with std IDF, poor explanations 👎
* [Examples in `esp-hal`](https://github.com/esp-rs/esp-hal/tree/main/examples) — examples for ESP32 HAL 👍
* [Rust Embedded Drivers Book](https://red.implrust.com/) — learn to write embedded drivers
* [`esp-hal` crate](https://docs.espressif.com/projects/rust/esp-hal/latest/) — see which peripherals are available and how to use them

Rust ESP32 Short tutorials:
* [Freenove ESP32 Rust](https://makuo12.github.io/Freenove-esp32-rust/)
* [ESP WiFi async example](https://github.com/arlyon/esp-wifi-async-example)
* [YouTube: Rust on ESP32-C3](https://www.youtube.com/playlist?list=PLkch9g9DEE0Lkm1LqcD7pZNDmXEczOo-a)
* [OTA with Rust](https://quan.hoabinh.vn/post/2024/3/programming-esp32-with-rust-ota-firmware-update)
* [Sending sensor data to Postgres](https://c410-f3r.github.io/thoughts/securely-sending-dht22-sensor-data-from-an-esp32-board-to-postgresql/)

Rust IDF (std):

* [`esp-idf-hal`](https://github.com/esp-rs/esp-idf-hal) — implements [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) on ESP-IDF for ESP32: Safe Rust wrappers for the drivers in the ESP IDF SDK. NOTE: it's a *community effort*!
* [`esp-idf-template`](https://github.com/esp-rs/esp-idf-template) — template project to use with `cargo generate`
* [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/)
* [ESP-IoT Solution Programming Guide](https://docs.espressif.com/projects/esp-iot-solution/en/latest/index.html)
* [ESP ZigBee Programming Guide](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32c3/index.html)
* [ESP-IDF Extension for VSCode](https://docs.espressif.com/projects/vscode-esp-idf-extension/en/latest/)

Code for ESP32, Rust:

* [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)
* [Awesome ESP Rust](https://github.com/esp-rs/awesome-esp-rust)
* [`embedded-hal` crate](https://docs.rs/embedded-hal/) — generic Hardware Abstraction Layer for Rust. `esp-hal` implements these traits within its drivers.
* [`esp-hal`](https://github.com/esp-rs/esp-hal) — `no_std` HAL for ESP32, officially supported

Code for ESP32, non-Rust:

* [Examples for IDF in C](https://github.com/espressif/esp-idf/tree/master/examples)
* [masoncj/ESP32 examples](https://github.com/masoncj/esp32-examples)

General:

* [Writing an OS in Rust](https://os.phil-opp.com/)
* [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust)
* [Embedonomicon](https://docs.rust-embedded.org/embedonomicon/): a deep dive into the implementation of the foundational crates: linker, symbols, and ABIs.

Lessons:w

* 🇷🇺 [AlexGyver](http://alexgyver.ru/)
* 🇷🇺 [NarodStream](https://narodstream.ru/programmirovanie-esp32/)

OSes and Frameworks:

* [Embassy: modern embedded framework](https://github.com/embassy-rs/embassy): async/await. HAL with batteries included. Clean, modern async code flow, great for I/O heavy or complex state machines. High productivity. Use case: IoT, network services.
* [TockOS](https://github.com/tock/tock): Microkernel. System calls. Cooperative userspace apps. Microkernel separates drivers (capsules) and applications (user-space). Ability to run dynamically loaded applications. Use case: wearable devices.
* [HubrisOS](https://github.com/oxidecomputer/hubris): Monolithic/Hybrid. Tasks known at compile time. Applications compiled directly into the monolithic kernel; relies on MPU for isolation.
* [RTIC](https://github.com/rtic-rs/rtic): for real-time applications. Realtime, interrupt-driven, hardware-accelerated via interrupt priorities (SRP protocol). Execution framework only; relies on separate HALs/PACs for hardware. Minimal overhead, fastest context switching, highly suitable for hard real-time control. Use case: Hard real-time control, motor control, tight timing loops, systems requiring minimal overhead.

For VSCode:

* `lldb` a native debugger extension based on LLDB
* `crates` to help manage Rust dependencies
* [Wokwi](https://wokwi.com/esp32) is an online ESP32 simulator. It has a free VSCode extension (renewable 30-day personal use)

# Other Controllers

Raspberri Pi:

* [Learn to write an embedded OS in Rust 🦀](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)

STM32F4:

* [STM32 Embedded Rust HAL](https://apollolabsblog.hashnode.dev/series/stm32f4-embedded-rust-hal)

## Connect the Board

Connect board to your computer. Verify, a tiny red control LED lights up.
The device should also expose its UART serial port over USB.
It will show up in `lsusb`:

```console
$ lsusb | grep USB
Bus 006 Device 035: ID 303a:1001 Espressif USB JTAG/serial debug unit
```

Find the device by id:

```console
$ ls -l /dev/serial/by-id
lrwxrwxrwx 1 root root .... usb-Espressif_USB_JTAG_serial_debug_unit_60:55:F9:C0:27:18-if00 -> ../../ttyACM0
```

The device will either be `/dev/ttyACM0` or `/dev/ttyUSB0`.
This depends on on the USB-to-serial implementation:
* ACM (Abstract Control Model) driver is used by boards with native USB support or CDC-ACM USB-serial chips.
  The microcontroller itself handles USB, or uses chips that implement the CDC class.
  ACM devices generally support higher speeds and are considered "proper" USB devices rather than converters.
* ttyUSB: USB-serial converter driver. Used by boards with dedicated USB-to-UART bridge chips 
  like CP2102, CH340, FTDI, etc. These are separate chips that convert USB to plain serial.



# Hardware Overview

## ESP Architecture and Support

Espressif chips have two different architectures:

* Xtensa: The ESP32 and ESP32-S series are based on the Xtensa architecture.
* RISC-V: The ESP32-C,H,P series are based on the RISC-V architecture.

Xtensa is **not yet officially supported by Rust**: because Rust uses LLVM as part of its compiler infrastructure, and LLVM does not yet support Xtensa.
However, there is a Rust fork with Xtensa support: <https://github.com/esp-rs/rust>.
It's support is Tier 3 (requires custom toolchain installation): <https://doc.rust-lang.org/nightly/rustc/platform-support/xtensa.html>
Watch this GitHub tracking issue for progress: <https://github.com/espressif/llvm-project/issues/4>

RISC-V, on the other hand, is open-source. Also, China is moving away from any US based IP as much as possible.
Xtensa is from Cadence (former Tensilica): a US company. In 2022, Espressif announced that it moves exclusively to RISC-V.

There is an even older ESP8266 chip. It is not supported by `esp-hal` for Rust. Forget it.


## `esp-hal` Ecosystem

The crates under the `esp-rs` organization include support for all ESP32 chips.

The `esp-hal` crate ties Rust to ESP chips: lets you initialize it and access drivers for the peripherals.
It implements traits from the Rust's `embedded-hal` Hardware Abstraction Layer, which itself is chip-agnostic.

See full [esp-hal documentation](https://docs.espressif.com/projects/rust/esp-hal/latest/) for your chip and see which peripherals are available.

Other crates:

* `esp-radio`: Wi-Fi, BLE, esp-now and low-level IEEE 802.15.4: basis for ZigBee and Thread.
* `esp-alloc`: heap allocation in `no_std` environment (where no OS is there to manage memory for you)
* `esp-println`: print and logging
* `esp-sync`: synchronization primitives
* `esp-storage`: storage utilities
* ... many more

Crates:

* `esp-*` repositories are focused on `no_std` applications: e.g. `esp-hal`
* `esp-idf-*` are focused on `std` apps: e.g. `esp-idf-hal`



# Toolchain and Build Target

A *toolchain* is a single installation of the Rust compiler. Rustup can install stable/beta/nightly toolchains,
as well as toolchains from alternative platforms (like ESP Xtensa forks).

Each toolchain has several components: like `rustc`, `cargo`, `rustfmt`, `rust-std`, `rust-src`.
You can install them using:

```console
$ rustup toolchain install nightly --component rust-docs
$ rustup component add rust-docs
```

More on toolchains and components: [rustup book](https://rust-lang.github.io/rustup/concepts/toolchains.html).

## RISC-V

Install the proper toolchain and add the `rust-src` component:

```console
$ rustup toolchain install stable --component rust-src
```

Install the target:

```console
$ rustup target add riscv32imc-unknown-none-elf # For ESP32-C2 and ESP32-C3
$ rustup target add riscv32imac-unknown-none-elf # For ESP32-C6 and ESP32-H2
```

See which targets does your Rust support:

```console
$ rustc --print target-list
```

## Xtensa

You'll need the fork of the Rust compiler. See here: <https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html#xtensa-devices>

Links:

* Old Rust-ESP32 dev container: <https://github.com/ctron/rust-esp-container/blob/master/Dockerfile>


For all chips (Xtensa *and* RISC-V):

* install `espup`: simplifies installing and maintaining the components required to build
* Run `espup install` to install the toolchain: Rust fork, nightly toolchain, LLVM fork, GCC toolchain

Instead of installing, you can use a Docker image: [espressif/idf-rust](https://hub.docker.com/r/espressif/idf-rust/tags):

```console
$ docker pull espressif/idf-rust:all_latest
```

But if you still want to install:
first of all, it is recommended to use `rustup` rather than your distro's package manager!
Your `rustup` will be able to determine which toolchain to use: see [rustup overrides](https://rust-lang.github.io/rustup/overrides.html).

But anyway:

```console
$ sudo apt install llvm-dev libclang-dev clang libuv-dev
$ cargo install cargo-espflash espflash ldproxy
$ espup install
```

Now source this file in every terminal:

```console
$ . $HOME/export-esp.sh
```

or use direnv's `.envrc`:

```bash
#!/bin/bash

# direnv:
# will automatically configure your environment
# see:
# * https://direnv.net/
# * https://github.com/direnv/direnv/wiki
# * https://github.com/direnv/direnv/blob/master/stdlib.sh

watch_file ~/export-esp.sh
. ~/export-esp.sh
```

Also, you'd need to set the toolchain for this folder:

```console
$ rustup override set esp
```

See which targets does your Rust support:

```console
$ rustc --print target-list
```


# Tooling

Tools for ESP32:

* `esp-generate` to generate a functional `no_std` project with most of the desired configurations pre-applied.
* `cargo generate` go generate a project for `std` using the [`esp-idf-template`](https://github.com/esp-rs/esp-idf-template).
* `espflash` is a serial flasher utility designed for Espressif SoCs and modules.
* `probe-rs` can flash, monitor, and also provides debugging capabilities: i.e. step, pause, inspect.
  It's a universal embedded debugging tool that works across many chips.
* `esp-config` is a tool to edit configuration options via a TUI (terminal GUI, i.e. ncurses).
  You can configure the parameters using env variables or `.cargo/config.toml`

```console
$ cargo install espflash --locked
$ cargo install esp-generate --locked
$ cargo install probe-rs --features cli,ftdi
$ cargo install esp-config --features=tui --locked
```

or use binstall for faster installation.

Also you might want to add:

* `minicom`: open a terminal with a USB-connected device
* `lldb` a native debugger extension based on LLDB
* `esptool`: communicate with the ROM bootloader
* `cargo-embed`: cargo-embed is the big brother of `cargo-flash`.
  It can flash a target, and it can also open an RTT terminal as well as a GDB server.
  Installed as a part of `probe-rs` tools. No need to install separately.
* `cargo-espflash`: a cargo subcommand wrapper around espflash. It does the same thing, but also has Cargo integration.
* `cargo install cargo-binstall`: to install Rust binaries without building from source



# Devcontainer

It is possible to install all the necessary tools in Docker.
The docs say that you can't flash ESP chips from the container, but I don't see why not:

```console
$ docker run --device=/dev/ttyUSB0 your-image espflash flash
$ docker run --privileged -v /dev:/dev your-image
```

You can create a devcontainer with all the tools you need.
See [./.devcontainer](./.devcontainer/).

Here's how you create a devcontainer that VSCode will recognize:

1. Install the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) vscode extension from Microsoft
2. Ctrl-P, "Dev Containers: new", select "Rust", wait for it to build
3. Open a "New Terminal"
4. When you hit `F5`, you'll run the application inside the container

The Dev Containers extension uses the files in the `.devcontainer` folder:
* `devcontainer.json` is a config that determines how your dev container gets built and started.
* optional `Dockerfile` or `docker-compose.yml`, to create your dev containers.
VSCode connects to a Visual Studio Code Server running inside of your new dev container.

You can create these files manually.
Or in VSCode, use "Remote Explorer / Dev Containers" view.

Now, in the bottom-left corner, you can ... what?


## VSCode

Rust analyzer can behave strangely without `std`.
Add this to `settings.json`:

```json
{
  "rust-analyzer.checkOnSave.allTargets": false
}
```

or devcontainer:

```js
{
    "customizations": {
        "vscode": {
            "settings": {
                "rust-analyzer.checkOnSave": false,
            },
        },
    },
}

```


# espflash

Start by generating a project with `esp-generate`. It will prompt you for your chip.

For flashing, you got two options:

* `espflash`: the default.
  Enable `esp-backtrace`
* `probe-rs`: Enables Real Time Transfer (RTT) based options and allows on chip debugging.
  Enable `defmt` and `panic-rtt-target`

Here's how you get your code up and running:
it will compile and flash.

```console
$ cargo run
$ cargo run --release
```

💡 By default espflash will use a baud-rate of 115200 which is quite conservative.
An easy way to increase the baud-rate is setting the environment variable `ESPFLASH_BAUD` to e.g. 921600
(or pass the `espflash flash -B` flag):

```toml
# .cargo/config.toml
[target.riscv32imc-unknown-none-elf]
runner = "espflash flash -B 921600 --monitor --chip esp32c3"

[env]
ESP_LOG="info"
ESPFLASH_BAUD=921600
```







## Bootloader

Upon power up, many embedded devices will just start executing code from an address in flash memory.
Espressif chips are a bit more complicated, and require some steps to setup flash memory. For this, we require a bootloader:
a simple application that sets up the aforementioned operations, then jumps to executing other code.

Espressif uses two bootloaders:

1. First Stage Bootloader (ROM Bootloader): burned into ROM, cannot be changed.
   It sets up architecture-specific registers,
   checks the boot mode (automatic/manual bootloader: see [boot mode selection](https://docs.espressif.com/projects/esptool/en/latest/esp32c6/advanced-topics/boot-mode-selection.html))
   and reset reason, and loads the second stage bootloader.
2. Second Stage Bootloader:
   Loads your application and sets up the memory(RAM, PSRAM or flash).
   Not technically required, but is advised because it allows OTA support.

   At the moment, only ESP-IDF Bootloader is supported as a second stage bootloader.
   Later, MCUBOOT support is planned.

# Flash

The Flash memory is flashed with a partition table at the default offset:
the second-stage bootloader uses it to know where to place the binary.
Other partitions can hold other applications and arbitrary data: calibration data, filesystems, parameter storage.

Each entry in the partition table has a name, type (app, data, ..), subtype and the offset in flash where the partition is loaded.
See docs: [default partition table and how to create a custom partition table](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c6/api-guides/partition-tables.html#creating-custom-tables).



# `std` vs `no_std`

When you might want to use bare metal (`no_std`):

* When you need a smaller app or smaller memory footprint
* Direct hardware control: if you need low-level device drivers or access to specialized hardware features.
  Because std adds abstractions that can make it harder to interact directly with the hardware.
* Real-time constraints, time-critical applications, low-latency response.
  Because std can introduce unpredictable delays and overhead that can affect real-time performance.
* Custom requirements: bare-metal allows more customization and fine-grained control over the behavior of an application, which can be useful in specialized or non-standard environments.

When you might want to use the Standard Library (`std`):

* Rich functionality: If your embedded system requires lots of functionality like support for networking protocols, file I/O, or complex data structures.
* Portability: write portable code that uses the `std` crate
* Rapid development: The `std` crate provides high-level features

So, there are two Rust paths for ESP32:

* `no_std` (bare metal): `esp-hal`, `esp-radio`. Direct hardware access, smaller binaries. Official.
* `std` (ESP-IDF wrapper): `esp-idf-*` crates - wraps Espressif's C framework. Heavier, easier networking/WiFi. Community-maintained.


### `std` on ESP32: IDF

Unlike most other embedded platforms, Espressif supports the Rust standard library.
Most notably, this means you'll have arbitrary-sized collections like `Vec` or `HashMap` at your disposal, as well as generic heap storage using `Box`.

Espressif provides a C-based development framework called `esp-idf` which has support for all Espressif chips
starting with the ESP32; note that this framework does not support the ESP8266.
ESP-IDF provides OS-like features.

`esp-idf` provides a [`newlib`](https://sourceware.org/newlib/) environment with enough functionality
to build the Rust standard library (std) on top of it. This is the approach that is being taken to enable std support on ESP devices.

You're also free to spawn new threads, and use synchronization primitives like `Arc` and `Mutex` to safely share data between them.
Still, memory is a scarce resource on embedded systems, and so you need to take care not to run out of it - threads in particular can become rather expensive.

Espressif provides a C-based development framework: [ESP-IDF](https://github.com/espressif/esp-idf), which provides a [newlib](https://sourceware.org/newlib/) environment that has enough functionality to build the Rust `std` on top of it.

When using `std`, you have access to a lot of `ESP-IDF` features: threads, mutexes, collections, random numbers, sockets, etc.

Services like Wi-Fi, HTTP client/server, MQTT, OTA updates, logging etc. are exposed via Espressif's open source IoT Development Framework, ESP-IDF.
It is mostly written in C and as such is exposed to Rust in the canonical split crate style:

* the [esp-idf-sys](https://github.com/esp-rs/esp-idf-sys) crate provides the actual `unsafe` bindings to the IDF development framework that implements access to drivers, Wi-Fi, and more
* the higher-level [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc) crate implements safe and comfortable Rust abstractions: it implements abstractions from [embedded-svc](https://github.com/esp-rs/embedded-svc): wi-fi, network, httpd, logging, etc
* [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal): implements traits from `embedded-hal` and other traits using the `esp-idf` framework: analog/digital conversion, digital I/O pins, SPI communication, etc.
* [esp-pacs](https://github.com/esp-rs/esp-pacs/tree/main/esp32c3) provices Peripheral Access Crates (PACs) if direct register manipulation is required. Generated from SVD files.

Note that `esp-idf-*` crates are *community effort*.
Espressif boasts that it supports Rust std, but in fact, it's the community that's built a Rust std environment
around the official IDF framework.


### `no_std` on ESP32

Using `no_std` may be more familiar to embedded Rust developers: it uses a subset of `std`: the `core` library.

See crates:

* [esp-hal](https://github.com/esp-rs/esp-hal): Hardware abstraction layer
* [esp-pacs](https://github.com/esp-rs/esp-pacs): Peripheral access crates
* [esp-wifi](https://github.com/esp-rs/esp-wifi): Wi-Fi, BLE and [ESP-NOW](https://www.espressif.com/en/solutions/low-power-solutions/esp-now) support
* [esp-alloc](https://github.com/esp-rs/esp-alloc):	Simple heap allocator
* [esp-println](https://github.com/esp-rs/esp-println):	`print!`, `println!`
* [esp-backtrace](https://github.com/esp-rs/esp-backtrace):	Exception and panic handlers
* [esp-storage](https://github.com/esp-rs/esp-storage):	Embedded-storage traits to access unencrypted flash memory



## New Project

### `no_std`

```console
$ esp-generate -c esp32c3
```

Examples:

```console
$ esp-generate -o stack-smashing-protection -o esp-backtrace -o wokwi -o log
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






# topics
# ESP32 Family Chips

Old ESPs are all Xtensa:

* ESP32 (Wifi, BT): the original flagship. Dual core.
* ESP32-S: newer version. Higher performance. New hardware.

  Got OTG support: can connect to USB directly!
  Makes ch340 redundant: this is why some ESP32-S3 boards are physically smaller.
  Some boards still include a separate CH340 (or CP210x) chip: useful if the native USB code crashes or the chip is in a sleep mode.

    * S2: signle-core for power efficiency. Only Wi-Fi.
    * S3: dual-core, WiFi, BT. AI features: supports vector instructions.

Newer ESP chips are all RISC-V.

* ESP32-C (compact, cost-effective, communication): WiFi, Bluetooth, Zigbee/Thread. Not as performant as S3, though, and less RAM.

    * ESP32-C6: 2.4Ghz WiFi 6, BT 5, Zigbee, Thread. 160 Mhz. Works with external flash. It also includes a 20Mhz extra low power core.
    * ESP32-C61: 2.4Ghz WiFi 6, BT 5. Ultra low power.
    * ESP32-C5: 2.4 and 5 GHz Wi-Fi 6, BT 5, Zigbee, Thread. Ultra low power.
    * ESP32-C3: 2.4Ghz WiFi, BT 5. 160 Mhz. 384Kb ROM, 400Kb SRAM. Crypto peripherals. Allows connections to flash.
    * ESP32-C2: 2.4Ghz WiFi, BT 5. 120 Mhz. 576Kb ROM, 272Kb SRAM. Only 14 GPIOs. Cheapest and smallest.

* ESP32-H (hibernate): 	Focus on Thread/Zigbee, no Wi-Fi. Has Bluetooth. Low power consumption in sleep mode.
* ESP32-P (performance): high-performance chip: up 400 MHz. FPU, AI features. No wireless. Newest chips.


## Hardware Terminology

* System-on-a-Chip: the chip package itself. Includes the CPU and its peripherals, all in one package.
  SoCs are primarily intended for integration into custom hardware designs.
* Module: chip on a board with some resistors, crystal oscillator, antenna, flash memory, EMI shield, ... — a ready-to-use solution.
  Common examples: WROOM, WROVER series
* Development Boards (Devkit): have a USB interface, voltage regulator, pin breakouts, boot and reset buttons.


## ESP32 Chip Marking

### ESP32

**ESP32** D 0 WD R2 H Q6 V3:

* `D`/`U`: Dual Core; `S`: Single Core
* In-package flash: `0`=None, `2`=2Mb, `4`=4Mb
* Connection. `WD`: Wi-Fi b/g/n + Bluetooth/Bluetooth LE dual mode
* In-package PSRAM. `R2`: 2 MB PSRAM
* `H`: High temperature
* Package. `Q6`: QFN 6*6; *N/A*: QFN 5*5
* `V3`: Chip revision v3.0 or newer

### ESP32-C3

**ESP32-C3** F H/N 4 X

* `F`: Flash. Has flash.
* `H/N`: Flash temperature: `H`: High Temperature, `L`: Low Temperature
* `4`: Flash size, Mb
* `AZ`: Other identification code

### ESP32-C6

**ESP32-C6** F H/N 4

* `F`: Flash. Has flash.
* `H/N`: Flash temperature: `H`: High Temperature, `N`: Normal Temperature
* `4`: Flash size, Mb




## Flash

Note that the internal flash has limited number of write cycles.
ESP32 flash can handle 100.000 cycles at minimum.

## Peripherals

While the CPU is responsible for executing program logic, peripherals are hardware components
that extend its capabilities: they allow the MCU to interact with the outside world by handling
inputs and outputs, communication, timing, and more.
This allows the CPU to focus on critical tasks while peripherals handle specialized functions independently.

*Offloading* refers to the practice of delegating certain tasks to hardware peripherals
instead of doing them directly in software via the CPU.
This improves performance, reduces power consumption, and enables concurrent operations.

For example:

* A UART peripheral can send and receive data in the background using DMA (Direct Memory Access), while the CPU continues processing other logic.
* A Timer can be configured to generate precise delays or periodic interrupts without CPU intervention.
* A PWM controller can drive a motor continuously without the CPU constantly toggling pins.

Offloading is a key design strategy in embedded systems to make efficient use of limited processing power.

Common types of peripherals:

* *GPIO* (General Purpose Input/Output)	Digital pins that can be configured as inputs or outputs to interact with external hardware like buttons, LEDs, and sensors.
* *UART* (Universal Asynchronous Receiver/Transmitter)	Serial communication interface used for sending and receiving data between devices, often used for debugging and connecting modules like Bluetooth.
* *SPI* (Serial Peripheral Interface)	High-speed synchronous communication protocol used to connect microcontrollers to peripherals like SD cards, displays, and sensors using a master-slave architecture.
* *I2C* (Inter-Integrated Circuit)	Two-wire serial communication protocol used for connecting low-speed peripherals such as sensors and memory chips to a microcontroller.
* *ADC* (Analog-to-Digital Converter)	Converts analog signals from sensors or other sources into digital values that the microcontroller can process.
* *PWM* (Pulse Width Modulation)	Generates signals that can control power delivery, used commonly for LED dimming, motor speed control, and servo actuation.
* *Timer*	Used for generating delays, measuring time intervals, counting events, or triggering actions at specific times.
* *RTC* (Real-Time Clock)	Keeps track of current time and date even when the system is powered off, typically backed by a battery.


## Temperature Sensor

**ESP32-C3** has one internal temperature sensor.

It also has 2 SAR ADCs for measuring analog signals from six channels (6 pins)./
It samples voltages with 12-bit sampling resolution.

* SAR ADC1: can measure from GPIO 0..4
* SAR ADC2: can measure from GPIO 5 only

**ESP32-C6** has 1 SAR:

* SAR ADC: can measure from GPIO 0..6


# Electronic

## LED and Resistors

LEDs: longer leg is usually the anode: "+".

Use the Ohm's law to choose a resistor:

> R = (Vs - Vf) / If.

"LED forward voltage" (Vf) is the voltage drop across an LED:
usually it is 1.8V..2.0V.

The remaning voltage must be dissipated on a resistor.

For most LEDs, the safe current is 20mA. Use 10mA if you want to be on the safe side.

For the GPIO, the safe current is 20mA, absolute max = 40mA.

You can find the Vf out using the "diode testing mode `-|◀-` on a multimeter —
or connect it to a power source (through a resistor!) and measure the voltage across it.

So, if your `Vs`=3.3V and your `Vf`=2.0V, and the `If`=10mA,
you'll need a `130Ω` resistor.

I've also seen `330Ω` and even `1kΩ` resistors: a safe choice for an unknown LED.


## Voltage Drop

How to drop some volts?

1. Voltage divider: you'll waste a lot of current
2. Use a diode: every diode drops 0.7V. It dissipates the energy as heat.
3. Stabilitron: for small loads
4. LDO-module: Low-Dropout Regulator. MP2315, LM3940, ... . They will still heat up.
5. Switching regulator: doesn't heat up

# GPIO

GPIOs get initialized as inputs or outputs:

```rust
// Init GPIO 2 as Output, with initial state set to HIGH.
let mut led = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());
```

note that every GPIO is a different type so you can't choose them dynamically —
unless you use a match, macro, or cargo features:

```rust
#[cfg(feature = "led-gpio8")]
let led = gpio::Output::new(peripherals.GPIO8, ...);
#[cfg(feature = "led-gpio9")]
let led = gpio::Output::new(peripherals.GPIO9, ...);
```

# PWM: Pulse Width Modulation

Microcontrollers have binary logic: HIGH and LOW, with no in-between values.

PWM can generate an analog signal: a rectangular wave with varying *duty cycle* ­— without using the processor.
See: [Pulse-Width Modulation](https://en.wikipedia.org/wiki/Pulse-width_modulation).

Coupled with a capacitor, you can control the output voltage gradually.
If you microcontroller outputs 3.3V, then a PWM with duty cycle = 50% produces 1.65V on average.

Two parameters for a PWM:

1. Frequency
2. Duty cycle

Technically, it's implemented like this:

* A counter. It counts from 0 to a specific *maximum value* (stored in a register), then starts over.
* A *compare value* (stored in a register). When `counter < compare value`, the signal stays HIGH. When exceeds, the signal goes LOW.

*PWM Resolution*: how precisely can the duty cycle be controlled: is determined by the number of bits in the PWM register.
Example:

* 4-bit resolution => 16 duty cycle levels
* 8-bit resolution => 256 duty cycle levels
* 10-bit resolution => 1024 duty cycle levels.


## LED-PWM and MCPWM

Kinds:

* LED-PWM: simple dimming. Because when PWM switching happens very often, the eye cannot see it!
* Motor Control PWM: with more advanced features like fault detection and synchronization signals

ESP32-C3 has a LED-PWM controller with 6 channels, 4 independent timers, and 14 bits of resolution.
It also supports gradual increase/decrease of duty cycles (for dimming).

ESP-C6 has a LED-PWM with 20 bits.
It also has a Motor Control PWM (MCPWM): designed for driving digital motors and smart light.

## Architecture

LED PWM registers are clocked by `APB_CLK` (note that the `APB_CLK` signal to the LED PWM has to be enabled first
by setting the `SYSTEM_LEDC_CLK_EN` field in the register `SYSTEM_PERIP_CLK_EN0_REG`).

> `APB_CLK` (Advanced Peripheral Bus Clock) is the clock for a system's peripherals,
> while `CPU_CLK` is the clock for the central processing unit itself.
> `APB_CLK` is highly dependent on the `CPU_CLK` source.
> The main difference is that `APB_CLK` is slower: it basically is `CPU_CLK` divided:
> This division is necessary because peripherals don't need the same high speed. It's also more power efficient.

The 4 timers in ESP32-C3/C6 are identical in their features and operation: `Timer0`, `Timer1`, `Timer2`, `Timer3`.
Every timer maintains its own timebase counter.
The four timers can be independently configured, though: clock divider, counter overflow, etc.

Each PWM generator selects one of the timers and uses the timer’s counter value as a reference to generate
its PWM signal.

Timers can choose a clock signal:

* `APB_CLK` — a peripheral clock that is derived from these and other sources. Configurable frequency.
* `PLL_CLK`: (320 MHz or 480 MHz): internal PLL clock
* `XTAL_CLK` (40 MHz): external crystal clock
* `XTAL32K_CLK` (32 kHz): external crystal clock
* `RC_FAST_CLK` (17.5 MHz by default): internal, less stable, fast RC oscillator with adjustable frequency
* `RC_FAST_DIV_CLK`: internal fast RC oscillator derived from RC_FAST_CLK divided by 256
* `RC_SLOW_CLK` (136 kHz by default): internal low RC oscillator with adjustable frequency

A timer applies a *division factor* to the clock to slow it down.
The `clk_div=1..1023`.

## How to Get a Specific Frequency

ESP32-C3's LEDC (PWM) module uses a fixed 80 MHz APB clock as the source.
To hit a target frequency (e.g. 24 kHz), it divides the clock by a prescaler `D` and then
counts up to `2^duty_resolution_bits - 1`.

> resulting_pwm_freq = clock_freq / (clk_div * 2^duty_resolution_bits)

You pick `clk_div` first, then solve for the closest integer `duty_resolution_bits`
that gets you near 24 kHz. Higher bits => finer control, but forces larger `clk_div`:
possible frequence error if `clk_div` can't be exact.

So, you need to pick two parameters: `clk_div` and `duty_resolution_bits`.

In `esp-hal`, the `clk_div` is hidden: it's calculated from the `clk_div` you choose.

```rust
    lstimer0.configure(ledc::timer::config::Config {
        clock_source: ledc::timer::LSClockSource::APBClk,
        frequency: Rate::from_khz(24),
        duty: ledc::timer::config::Duty::Duty5Bit,  // choose
    }).unwrap();
```

Your goal is to get as close to the desired frequency as possible, using this formula:

> duty_resolution_bits = log2( clock_freq / ( desired_pwm_freq * clk_div ) ), where
> `clk_div`=1..1023.

* The lowest resolution is achieved with maxing out `clk_div=1023`.
* The highest resolution is achieved with `clk_div=1` (no division).

For 24 Khz: low=2, high=12. Choose any number in between.

## Example

Initialize:

```rust
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
    duty: ledc::timer::config::Duty::Duty10Bit,
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
```

Change duty cycle:

```rust
channel0.set_duty(30); // 30%
```

Fade-in, fade-out using hardware support for gradual duty cycle changes:

```rust
// PWM has `start_duty_fade()`: gradually changes from one duty cycle percentage to another.
// Fade in
channel0.start_duty_fade(75, 100, 500).unwrap();
while channel0.is_duty_fade_running() {} // wait

// Fade out
channel0.start_duty_fade(100, 75, 500).unwrap();
while channel0.is_duty_fade_running() {} // wait
```
# SPI

SPI is de-factor standard for synchronous serial communication in embedded systems.
Originally invented by Motorola in ~1980s.
Allows interfacing with peripheral chips, LCD displays, ADC/DAC converters, flash, EEPROM, and other chips.

SPI follows a master–slave architecture, where a master device orchestrates communication
with one or more slave devices by driving the clock and chip select signals.

It uses 4 wires to support full duplex (FDX).
In contrast to 3-wire variants which are half-duples (HDX): one direction at a time.

SPI provides higher throughput than I²C or SMBus, but it requires more pins.

SPI is different from SSI: SSI employs
[differential signalling](https://en.wikipedia.org/wiki/Differential_signalling) (differential pair)
and provides only a single simplex communication channel.

## Logic Signals

Commonly, SPI has four logic signals:

* **S̅S̅**: Slave Select. Master sends it to select slave chip to communicate with.  Active-low signal.
* **SCLK**: Serial Clock. Clock signal from master.
* **MOSI**: Master Out Slave In. Serial data output from master.
  MOSI on a master outputs to MOSI on a slave.
* **MISO**: Master In Slave Out. Serial data output from slave.
  MISO on a slave outputs to MISO on a master.

So, every chip on the SPI bus shares 3 wires (SCLK, MOSI, MISO) with all other chips,
but the *S̅S̅* wire needs to be separate for every chip: one pin for every peripheral.

## Chip Select

How chip-select works: when the *chip select* pin is held in the inactive state,
the chip remains "deaf" and pays no heed to changes in the state of its other input pins:
it holds its outputs in the high impedance state (electrically disconnected),
so other chips can drive those signals.
When the chip select pin is held in the active state, the chip or device assumes that
any input changes it "hears" are meant for it and responds as if it is the only chip on the bus.

This is called "tristate output": a *digital buffer* that has three stable states:
a high voltage output state (logical 1), a low output state (logical 0),
and a high-impedance (Hi-Z) state. In the Hi-Z state, the output of the buffer is effectively
disconnected from the subsequent circuit.
(A *buffer* is a circuit that "copies" the input without drawing current.)

To begin communication, the SPI master first selects a slave device by pulling its S̅S̅ low.
(The bar above S̅S̅ indicates it is an active low signal, so a low voltage means "selected",
while a high voltage means "not selected")

Caveat: All S̅S̅ signals should start HIGH before sending initialization messages to any slave.
Either configure your S̅S̅ GPIOs to be initially HIGH, or add a pull-up resistor on each S̅S̅,
to ensure that all S̅S̅ signals are initially high.

## Data Transfer

Each device internally uses a [shift register](https://en.wikipedia.org/wiki/Shift_register) for serial communication,
which together forms an inter-chip circular buffer. (A *shift register* is a cascade of flip-flops:
latches with two stable states that can store a bit of information. The cascade shares the clock signal,
which causes the data stored in the system to shift from one location to the next.
By connecting the last flip-flop back to the first, the data can cycle in the register.)

Data is usually shifted out with the most-significant bit (MSB) first.

## Clock

During each SPI clock cycle, full-duplex transmission of a single bit occurs.
The master sends a bit on the MOSI line while the slave sends a bit on the MISO line,
and then each reads their corresponding incoming bit. This sequence is maintained even when
only one-directional data transfer is intended.

The Master must also configure the clock polarity and phase with respect to the data.
Motorola called this CPOL and CPHA (Clock POLarity and Clock PHAse).

Two options for CPOL:

* `CPOL=0`: a clock which idles at the logical low voltage.
* `CPOL=1`: a clock which idles at the logical high voltage.

Two options for CPHA:

* `CPHA=0`: The first data bit is output *immediately* when S̅S̅ activates.
  Data bits are sent when SCLK transitions *to* idle.
  Sampling occurs when SCLK transitions *from* idle.
* `CPHA=1`: The first data bit is output on SCLK's first clock edge after S̅S̅ activates.
  Data bits are sent when SCLK transitions *from* idle.
  Sampling occurs when SCLK transitions *to* idle.

The combinations of polarity and phases are referred to by these "SPI mode" numbers:
with CPOL as the high order bit and CPHA as the low order bit:

* SPI mode = 0: CPOL=0, CPHA=0 (send=falling SCLK, and when S̅S̅ activates; sample=rising SCLK)
* SPI mode = 1: CPOL=0, CPHA=1 (send=rising SCLK; sample=falling SCLK)
* SPI mode = 2: CPOL=1, CPHA=0 (send=rising SCLK, and when S̅S̅ activates; sample=falling SCLK)
* SPI mode = 3: CPOL=1, CPHA=1 (send=fallking SCLK; sample=rising SCLK)

However, because MOSI and MISO signals are usually stable,
devices may sample data at different points in that half cycle, despite the specification.

## Interrupt from Slave to Master

SPI slaves sometimes use an out-of-band signal (another wire) to send an interrupt signal to a master.

Examples: sensors, real-time clock chips, SDIO (SD card), audio jack insertions.

## Bus Topologies

SPI can communicate with multiple slaves.

* Multidrop configuration: each slave has its own S̅S̅.
  This is the way SPI is normally used.
* Daisy chain: first slave's output is connected to to the second slave's input, ...,
  until the final slave, whose output is connected back to the master's input.
  And they share S̅S̅.
  This effectively merges their shift registers.
* Expander configurations: use SPI-controlled addressing units to add chip selects
  using demultiplexers.


## Variations

* Extra timing: Devices often require extra clock idle time: before the first clock, or after the last one, or between a command and its response.
* Dual SPI, Quad SPI: use additional data lines to transfer more bits at a time
* DDR: Transfer 2 bits per clock cycle

# IDF (std)

## RTOS Watchdog (IDF)

In `std` IDF, your tasks, including the `main()` function, must yield control
to FreeRTOS so that it can run other tasks.

If you don't, after 5s a watchdog will fire and kill the app with this message:

```
E (21573) task_wdt: Task watchdog got triggered. The following tasks/users did not reset the watchdog in time:
E (21573) task_wdt:  - IDLE (CPU 0)
E (21573) task_wdt: CPU 0: main
```

## WiFi (IDF)

When using IDF WiFi, it tries to register the chip (via DHCP client)
as "espressif", so the chip will often be available using hostname "espressif".

You can customize the hostname using `sdkconfig.defaults`:

  CONFIG_LWIP_LOCAL_HOSTNAME="esp32c3"

See all the options:
[Configuration Options Reference](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-reference/kconfig-reference.html)





# tutorials/a01-no_std-blinky-button
# New Project

Espressif used to support `cargo-generate`, but now they insist on using
the [`esp-generate`](https://github.com/esp-rs/esp-generate/) tool for `no_std` applications.

NOTE: if using a devcontainer, don't execute these commands on the host machine!
They will attempt to build the crate, which includes downloading all dependencies.

```console
$ esp-generate --chip esp32c3 -o wokwi --flashing-espflash esp-backtrace <project-name>
$ esp-generate
```

You can already build and flash:

```console
$ cargo build
$ cargo run
```

You can use `cargo run` because of `.cargo/config.toml`, which configures the build target and the runner:

```toml
[target.riscv32imc-unknown-none-elf]
runner = "espflash flash -B 921600 --monitor --chip esp32c3"
```

It will have a `rust-toolchain.toml`:

```toml
[toolchain]
channel    = "stable"
components = ["rust-src"]
targets = ["riscv32imc-unknown-none-elf"]
```





# tutorials/a01-no_std-blinky-button/src/bin


# tutorials/a01-no_std-blinky-button/src/bin/main.rs

```rust
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

```





# tutorials/a02-dma/src/bin


# tutorials/a02-dma/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::panic;

use esp_backtrace as _;
use esp_println::{print, println};
use esp_hal::{
    clock::CpuClock,
    time::Rate,
    delay::Delay,
    main,
    // DMA
    dma::{DmaRxBuf, DmaTxBuf},
    dma_buffers,
    spi::{
        master::{Config, Spi},
        Mode,
    },
};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Select pins
    //
    // NOTE: To make testing easy, connect GPIO4 to GPIO2.
    // This way the data we send is also the data we receive.
    let sclk = peripherals.GPIO0;  // Clock signal
    let miso = peripherals.GPIO2;  // Master -> Slave
    let mosi = peripherals.GPIO4;  // Slave -> Master
    let cs = peripherals.GPIO5;    // Chip select. HIGH=Ignore, LOW=select

    // DMA Init
    // The DMA peripheral can perform memory transfers in parallel to the work of the processor:
    // i.e. while your program is doing something else.
    // The peripheral needs two buffers allocation in the ESP memory: 3200 bytes in our case.
    // It also needs some space for descriptors (linked list that DMA uses internally for transfer).
    let dma_channel = peripherals.DMA_CH0;
    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(32000);
    let mut dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let mut dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    // Configure SPI to use DMA.
    // We call `.with_dma()` on the SPI driver to make it use DMA
    let mut spi = Spi::new(
        // Note that SPI is a peripheral. ESP32 has SPI0, SPI1, SPI2
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(100))  // SPI frequency for DMS
            .with_mode(Mode::_0),  // SPI Mode: CPOL=0, CPHA=0
        )
        .unwrap()
        .with_sck(sclk)  // pin
        .with_mosi(mosi) // pin
        .with_miso(miso) // pin
        .with_cs(cs)     // pin
        .with_dma(dma_channel);  // use DMA

    let delay = Delay::new();

    // Populate the tx_buffer with data to send
    dma_tx_buf.as_mut_slice().fill(0x42);

    loop {
        // Initiate DMA transfer.
        // It will proceed in the background.
        //
        // Note that the buffers and the driver *move* into the `transfer` object
        // and are inaccessible during the transfer.
        // We'll get them back when the transfer is over.
        println!("Starting DMA transfer: {} bytes", dma_tx_buf.len());
        let transfer = match spi
            .transfer(dma_rx_buf.len(), dma_rx_buf, dma_tx_buf.len(), dma_tx_buf)
            .map_err(|e| e.0) {
            Ok(t) => t,
            Err(e) => {
                println!("DMA transfer failed");
                panic!("DMS error: {e:?}");
            }
        };
        // TODO: I don't know why Wokwi stops here. Maybe DMA is not implemented?
        println!("Started DMA transfer");

        // ... The CPU can do other things while the transfer is taking place ... //

        // ❗ You may think that using DMA is *always* preferable.
        // In fact, not: Setting up a DMA transfer consumes more CPU cycles than setting up a blocking transfer.
        // Especially if the amount of data is small, or if the CPU does nothing else besides waiting,
        // it's preferable to use a blocking transfer.

        // Let's wait
        while !transfer.is_done() {
            print!(".");
            delay.delay_millis(100);
        }
        // if the transfer isn't completed this will block
        (spi, (dma_rx_buf, dma_tx_buf)) = transfer.wait();

        // Done transfer
        println!(
            "Received {:x?} .. {:x?}",
            &dma_rx_buf.as_slice()[..10],
            &dma_rx_buf.as_slice().last_chunk::<10>().unwrap()
        );

        // Pause
        delay.delay_millis(2500);
    }
}

```





# tutorials/a03-no_std-http-client/src/bin


# tutorials/a03-no_std-http-client/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

// Read constants from env at compile time (!)
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

use esp_backtrace as _;
use esp_println::{print, println};
use esp_hal::{
    clock::CpuClock,
    time::{self, Duration, Instant},
    main,
    // TimerGroup for esp-rtos (required by esp-radio)
    timer::timg::TimerGroup,
    // For the heap allocator
    ram,
    // Random
    rng::Rng,
};

// ESP WiFi
use esp_radio::wifi::{ClientConfig, AuthMethod, ModeConfig, ScanConfig};



// smoltcp: A TCP/IP stack designed for bare-metal, real-time systems without a heap.
use smoltcp::{
    self,
    iface::{SocketSet, SocketStorage},
    wire::{DhcpOption, IpAddress},
    socket::Socket,
};
use core::net::Ipv4Addr;

// Non-async Networking primitives for TCP/UDP communication
// This is basically just ripped out of esp-wifi.
// Why do we need to use this one?
// I don't know where the code from `esp-wifi` been moved. It's not in the `esp-radio`.
use blocking_network_stack::Stack;

// Import traits.
// Otherwise `socket` won't have `read()` and `write()` methods.
use embedded_io::{Read, Write};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();
extern crate alloc;

#[main]
fn main() -> ! {
    // Init hardware
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // esp-generator: init ESP heap allocator
    // ESP32 has fragmented RAM regions. Here we are manually carving out chunks for the allocator.
    // Multiple separate heap allocators:
    // - 64K in reclaimed RAM: memory freed after boot (was used by bootloader).
    // - 36KB in default DRAM
    // - 66K in uninitialized DRAM (section of Data RAM that is not initialized at boot)
    // Other available regions: rtc_fast, rtc_slow, persistent, zeroed, reclaimed
    esp_alloc::heap_allocator!(#[ram(reclaimed)] size: 64 * 1024);
    esp_alloc::heap_allocator!(size: 36 * 1024);
    // Commented out because my ESP32C3 does not have DRAM2
    // esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    // Prepare RTOS: esp-rtos. A minimal scheduler.
    // It provides executors to enable running async code, tasks,
    // and implements the necessary capabilities (threads, queues, etc.) required by esp-radio.
    // - Take ownership of a timer grup and a software interrupt
    // - Start RTOS scheduler using that timer and interrupt
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);


    //=== WiFi
    // Init WiFi
    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    let (mut wifi_controller, interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    // Disable power saving on WiFi
    wifi_controller
        .set_power_saving(esp_radio::wifi::PowerSaveMode::None)
        .expect("Failed to set WiFi power save mode = off");

    // Configure WiFi client
    let client_config = ModeConfig::Client(
        ClientConfig::default()
            .with_ssid(SSID.into())
            .with_password(PASSWORD.into())
            .with_auth_method(AuthMethod::None),
    );
    wifi_controller.set_config(&client_config).expect("Failed to set WiFi config");

    // WiFi start
    wifi_controller.start().expect("Failed to start WiFi");
    println!("WiFi started: {:?}", wifi_controller.is_started());

    // WiFi: scan for networks.
    // NOTE: scan_with_config() is a blocking scan. It will return the list of found networks.
    println!("Starting Wifi scan...");
    let scan_config = ScanConfig::default()
        // .with_ssid(SSID).  // only scan for 1 SSID
        .with_max(10);        // max networks to return
    let res = wifi_controller.scan_with_config(scan_config).expect("Wifi scan failed");
    for ap in res {
        println!("Found WiFi: {:?}", ap);
    }
    println!("WiFi capabilities: {:?}", wifi_controller.capabilities().unwrap());

    // Connect to WiFi
    // NOTE this method is non-blocking. Loop over is_connected() and wait.
    wifi_controller.connect().expect("Connect failed");
    loop {  // wait until connected
        match wifi_controller.is_connected() {
            Ok(true) => break,
            Ok(false) => {}
            Err(err) => {
                panic!("WiFi error: {:?}", err);
            }
        }
    }
    println!("WiFi connected: {:?}", wifi_controller.is_connected().expect("WiFi failed to connect"));



    // === TCP/IP
    // Init smoltcp: A TCP/IP stack designed for bare-metal, real-time systems without a heap.
    // STA: Statin-Mode WiFi devices (i.e. not AP)
    let mut device = interfaces.sta;
    // Current time
    let timestamp = smoltcp::time::Instant::from_micros(
        esp_hal::time::Instant::now().duration_since_epoch().as_micros() as i64,
    );
    // smoltcp interface
    let iface = smoltcp::iface::Interface::new(
        // Configure it with the actual device's MAC address
        smoltcp::iface::Config::new(smoltcp::wire::HardwareAddress::Ethernet(
            smoltcp::wire::EthernetAddress::from_bytes(&device.mac_address()),
        )),
        &mut device,
        timestamp,
    );

    // DHCP: Prepare a DHCP client (socket)
    // Socket set: space for storing sockets. N=3. Static allocation at compile time.
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let mut socket_set = SocketSet::new(&mut socket_set_entries[..]);
    // Init a DHCP socket and move it into the socket set.
    socket_set.add({
        let mut dhcp_socket = smoltcp::socket::dhcpv4::Socket::new();
        dhcp_socket.set_outgoing_options(&[DhcpOption{
            // DHCP Option 12: hostname
            kind: 12,
            data: b"esp-radio",
        }]);
        dhcp_socket
    });
    // Wait for getting an ip address
    let rng = Rng::new();
    let now = || Instant::now().duration_since_epoch().as_millis();

    // Create a WiFi stack using smoltcp iface, WiFi device, and socket set.
    // - random() is needed to pick the local port for the DHCP client.
    // - now: fn to get the current time, ms since epoch. It has to be monotonic, I guess.
    let stack = Stack::new(iface, device, socket_set, now, rng.random());

    // DHCP: get an IP address
    println!("Getting IP address...");
    loop {
        stack.work();
        if stack.is_iface_up() {
            println!("DHCP got IP address: {:?}", stack.get_ip_info().expect("DHCP failed to get an IP"));
            break;
        }
    }




    //=== Connected
    // Now we are connected.
    println!("Connected");

    // Prepare a socket to be used for HTTP requests.
    // It needs two buffers.
    let mut rx_buffer = [0u8; 1536];
    let mut tx_buffer = [0u8; 1536];
    let mut socket = stack.get_socket(&mut rx_buffer, &mut tx_buffer);

    loop {
        // Let the TCP stack make progress.
        // Pumps the network stack - processes packets, handles retransmissions, updates TCP state machines.
        // Make sure to call this function regularly.
        // It delegates to WifiStack::work()
        socket.work();

        // Open a TCP socket.
        // Use a pre-known IP address of the server.
        socket
            .open(IpAddress::Ipv4(Ipv4Addr::new(142, 250, 185, 115)), 80)
            .unwrap();
        // Send some HTTP bytes
        socket
            .write(b"GET / HTTP/1.0\r\nHost: www.mobile-j.de\r\n\r\n")
            .unwrap();
        // Actually sends the data.
        // write() was non-blocking: it only wrote to the buffer.
        socket.flush().unwrap();

        // Keep reading from the socket; timeout after 20 seconds.
        let deadline = time::Instant::now() + Duration::from_secs(20);
        let mut buffer = [0u8; 512];
        while let Ok(len) = socket.read(&mut buffer) {
            let received = unsafe { core::str::from_utf8_unchecked(&buffer[..len]) };
            print!("Recv: {}", received);

            if time::Instant::now() > deadline {
                println!("Timeout");
                break;
            }
        }

        // Done with the socket
        socket.disconnect();

        // For 5 more seconds, let the stack do its work.
        let deadline = time::Instant::now() + Duration::from_secs(5);
        while time::Instant::now() < deadline {
            socket.work();
        }
    }

}

```





# tutorials/a03-no_std-http-client


# tutorials/a03-no_std-http-client/.gitignore

```
# will have compiled files and executables
debug/
target/
.vscode/
.zed/
.helix/

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb

# RustRover
#  JetBrains specific template is maintained in a separate JetBrains.gitignore that can
#  be found at https://github.com/github/gitignore/blob/main/Global/JetBrains.gitignore
#  and can be added to the global gitignore or merged into this file.  For a more nuclear
#  option (not recommended) you can uncomment the following to ignore the entire idea folder.
#.idea/

```





# tutorials/a04-std-idf-http-client
# Generate Project

Unlike `no_std` projects, ESP-IDF projects (`std`) are generated with

```console
$ cargo generate esp-rs/esp-idf-template cargo
```

Additional Configuration Files:
* `build.rs` - Cargo build script. Here: sets environment variables required for building.
* `.cargo/config.toml` - sets the target architecture,
  a custom runner (`espflash`) to flash and monitor the device, and controls build details.
* `sdkconfig.defaults` - overrides ESP-IDF specific parameters such as stack size, log level, etc.

Advice: To save disk space and download time, set the toolchain directory to `global`.
Otherwise, each new project/workspace will have its own instance of the toolchain installed on your computer:

```toml
// .cargo/config.toml
[env]
# ...
ESP_IDF_TOOLS_INSTALL_DIR = { value = "global" } # add this line
```




# Hardware: Addressable LEDs

Addressable LEDs have a single shared data line.
The data signal is sent in a daisy chain, where each LED receives its data, processes it,
and passes the rest along to the next LED.

Data transfer protocol: "single NZR communication mode" ([Non-return-to-zero](https://en.wikipedia.org/wiki/Non-return-to-zero)):

* Zero: short pulse + long pause (at least 2x long)
* One: long pulse + short pause (at least 2x short)
* Reset: no transmission for at least 50μs

Data packets: 24bit = RGB for one LED.
It's actually GRB because G7 bit goes first.

The bits:

> G76543210R76543210B76543210

Parameters for WS2812B:
* Read frequency: at least 400Hz/s.
* Length: at least 5m without any increase circuit
* When the refresh rate is 30fps, cascade number are not less than1024 points.
* Send data at speeds of 800Kbps (mind the interference!)

In the datasheet, the NZR timings are called:
* `T0H`, `T0L`. When sending zero: time for the high pulse, time for the low pulse.
* `T1H`, `T1L`. When sending one: time for the high pulse, time for the low pulse.
* `Treset`: time to wait until reset. > 50μs

Timings:
* `T0H`: 0.4μs ± 150ns ; `T0L`: 0.85μs ± 150ns
* `T1H`: 0.8μs ± 150ns ; `T1L`: 0.45μs ± 150ns

So 1 diode takes ~ `TH+TL = 1.25μs ± 600ns` to refresh.
1000 diodes can be controlled in 1.25ms. Nice.

The ESP32-C3's RMT peripheral, however, only has 192*3 bits of memory on one channel.
This limits us to 256 LEDs.





# tutorials/a04-std-idf-http-client/src


# tutorials/a04-std-idf-http-client/src/main.rs

```rust
// Configuration.
// Is picked up *at compile time* by `build.rs` from `cfg.toml`
// $ cargo add toml-cfg
// This macros defines variable: CONFIG
#[toml_cfg::toml_config]
pub struct Config { // cfg.toml
    #[default("Wokwi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

use std::time::Duration;

// IDF HAL and services
use esp_idf_hal::{
    prelude::*,
    delay::FreeRtos,
    gpio::PinDriver,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
};

// Provides anyhow::Error for easy error handling:
//   anyhow::Result<T> = anyhow::Result<T, Error>
//   bail!("Missing attribute: {}", missing); = return Err(anyhow!(..));
//   ensure!(user == 0, "only user 0 is allowed");
use anyhow::{bail, Result};

// Our libraries
use a04_std_idf_http_client::{
    blinky_led, http_client, rgb_led::{self, RGB8}, wifi
};

// App: will connect to WiFi, blink the LED green/red to show the outcome.
// Will blink yellow while connecting.
fn main() -> Result<()> {
    // Required. Makes sure that patches to ESP-IDF are linked to the final executable.
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // We're running on top of FreeRTOS now.
    let sysloop = EspSystemEventLoop::take()?;

    // Init NVS (non-volatile storage).
    // WiFi driver uses it to store calibration data.
    // Without it, WiFi can't optimize its radio performance.
    let _nvs = EspDefaultNvsPartition::take()?;

    // Peripherals/devices/components
    let peripherals = Peripherals::take().unwrap();
    let blue_led_pin = peripherals.pins.gpio8;
    let rgb_led_pin = peripherals.pins.gpio4;  // addressable LED
    let rgb_led_rmt0 = peripherals.rmt.channel0;  // generate signals (for RGB LEDs)

    // Driver: Simple GPIO LED
    // In IDF, we use `PinDriver` to control the pin. Not the bare-metal `Output`.
    let mut blue_led_output = PinDriver::output(blue_led_pin)?;
    blue_led_output.set_low()?;
    let mut blink = blinky_led::BlinkyLed::new(blue_led_output)?;

    // Driver: RGB LED
    let mut led = rgb_led::WS2812RMT::new(rgb_led_pin, rgb_led_rmt0)?;

    // Log
    log::info!("Board starting...");

    //=== RGB LED control ===//

    // LED: blink yellow.
    // Use our module: rgb_led
    for _ in 0..3 {
        // NOTE: This is inefficient. WS2812Z protocol supports animations.
        led.set_pixel(RGB8::new(50, 50, 0))?;

        // Sleep using FreeRTOS.
        // It hopefully uses proper wait.
        FreeRtos::delay_ms(200);

        // Turn off
        led.set_pixel(RGB8::new(0, 0, 0))?;
        FreeRtos::delay_ms(200);
    }

    // LED: Control a 64x64 LED matrix of addressable LEDsw
    log::info!("LED Heart 64x64");
    led.heart64x64()?;

    //=== WiFI connect ===//

    // Get CONFIG: the variable is set by `toml_cfg`
    let app_config = CONFIG;

    // Connect to WiFi
    // Connect to the Wi-Fi network
    let _wifi = match wifi::new(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        // The modem peripheral
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => inner,
        Err(err) => {
            // Red!
            led.set_pixel(RGB8::new(50, 0, 0))?;
            bail!("Could not connect to Wi-Fi network: {:?}", err);

            // We panicked.
            // ESP32 will restart and try again.
        }
    };

    // Yield control to FreeRTOS.
    // Otherwise it might think we're dead.
    FreeRtos::delay_ms(0);

    //=== HTTP Load Page ===//

    // We'll keep re-loading the page.
    // Every time it changes (the content length) — we blink.

    let mut last_content_len: u64 = 0;

    loop {
        const URL: &str = "https://example.com/";

        // HTTP Client.
        // Load a webpage.
        let (content_len, body )= http_client::get_page(URL)?;
        log::info!("HTTP GET: Content-Length={content_len} Body={body}");

        // Blink if content length changed
        if last_content_len != content_len {
            blink.blink(6, Duration::from_millis(300), Duration::from_millis(100))?;
        }
        last_content_len = content_len;

        // We sleep; RTOS runs other tasks.
        // WARNING: if we do not yield control for >5s, the following error pops up:
        //  E (21573) task_wdt: Task watchdog got triggered. The following tasks/users did not reset the watchdog in time:
        //  E (21573) task_wdt:  - IDLE (CPU 0)
        //  E (21573) task_wdt: CPU 0: main
        FreeRtos::delay_ms(1000); // let other tasks run
    }
}

```



# tutorials/a04-std-idf-http-client/src/lib.rs

```rust
pub mod rgb_led;
pub mod wifi;
pub mod http_client;
pub mod blinky_led;

```



# tutorials/a04-std-idf-http-client/src/rgb_led.rs

```rust
use anyhow::Result;
use core::time::Duration;
use esp_idf_hal::{
    gpio::OutputPin,
    peripheral::Peripheral,
    rmt::{config::TransmitConfig, FixedLengthSignal, PinState, Pulse, RmtChannel, TxRmtDriver},
};

pub use rgb::RGB8;

// Driver for the WS2812 LED.
// Using the RMT (Remote Control) channel:
//   can generate many periodic sequences with minimal CPU intervention.
//   That is, send 10101001 binary sequences with specific lengths.
// See also:
// - https://crates.io/crates/ws2812-esp32-rmt-driver
// - https://crates.io/crates/ws2812-spi
pub struct WS2812RMT<'a> {
    // RTM driver
    rmt: TxRmtDriver<'a>,

    // Pre-configured pulse lengths for WS2812
    pulse: PulseConfig,
}

// Pre-configured pulse lengths for WS2812
struct PulseConfig {
    // Transmit 0: short HIGH + long LOW
    t0h: Pulse,
    t0l: Pulse,
    // Transmit 1: short LOW + long HIGH
    t1h: Pulse,
    t1l: Pulse,
}

// This is mostly copied from the std training:
//   https://github.com/esp-rs/std-training/blob/main/common/lib/rgb-led/src/lib.rs
impl<'d> WS2812RMT<'d> {
    // Init: led + RMT peripheral
    pub fn new(
        led: impl Peripheral<P = impl OutputPin> + 'd,
        rmt: impl Peripheral<P = impl RmtChannel> + 'd,
    ) -> Result<Self> {
        // RMT works with ticks: every pulse's length is given in tick counts.
        // ESP32-C3 has an internal RMT clock running at 80 Mhz
        // Clock divider: slow down the internal RMT clock (80 Mhz on ESP32-C3) to get the timing resolution you need.
        // This is because RMT works with ticks, not with milliseconds.
        // So with clk_div=2: one tick = 2/80 Mhz = 0.025μs = 25ns.
        // Because pulse length is a u16, you can generate pulses with lengths 25ns..1600μs
        // With Pulse length in ticks being a u16, you can generate signals between 25ns..819μs
        // Btw, the range for RMT:
        // - min: clk_div=1  : 12.5ns .. 409μs
        // - max: clk_div=255:    3μs .. 104ms
        let config = TransmitConfig::new().clock_divider(2);

        // RTM Tx driver
        let rmt_tx_driver = TxRmtDriver::new(rmt, led, &config)?;

        // Get the speed of the internal clock: used for calculating the number of ticks
        let ticks_hz = rmt_tx_driver.counter_clock()?;
        log::info!("hz={ticks_hz}");


        // Pre-calculate the actual pulse lengths.
        // They depend on the clock freq and the clk_div clock divider factor.
        let pulse_config = PulseConfig{
            t0h: Pulse::new_with_duration(ticks_hz, PinState::High, &ns(400))?,
            t0l: Pulse::new_with_duration(ticks_hz, PinState::Low , &ns(850))?,
            t1h: Pulse::new_with_duration(ticks_hz, PinState::High, &ns(800))?,
            t1l: Pulse::new_with_duration(ticks_hz, PinState::Low , &ns(450))?,
        };

        // RMT Tx driver, configured.
        Ok(Self {
            rmt: rmt_tx_driver,
            pulse: pulse_config,
        })
    }

    // Sed the color of exactly one pixel.
    pub fn set_pixel(&mut self, rgb: RGB8) -> Result<()> {
        // Color: convert to GRB encoding
        let color: u32 = ((rgb.g as u32) << 16) | ((rgb.r as u32) << 8) | rgb.b as u32;

        // Prepare the lengths: 24 bits of data, encoded.
        let mut signal = FixedLengthSignal::<24>::new();
        let p = &self.pulse;
        for i in (0..24).rev() {
            // Get the bit
            // TODO: More efficiently: shift "color >> 1" every iteration, destructuring the value
            let bit = (color & 1<<i) != 0;
            // Convert to a pair of Pulses: level + duration
            let (high_pulse, low_pulse) = if bit { (p.t1h, p.t1l) } else { (p.t0h, p.t0l) };
            // Add to signal
            signal.set(23 - i as usize, &(high_pulse, low_pulse))?;
        }

        // Send the signal
        self.rmt.start_blocking(&signal)?;

        // Done
        Ok(())
    }

    // Draw a heart on a 64x64 LED array.
    // This has nothing to do with board test; it's just a fun experiment
    pub fn heart64x64(&mut self) -> Result<()> {
        // NOTE: This HUGE array requires a bigger stack.
        // Size: 64 * 24 = 1.5K
        // Better allocate it in the memory
        const R: RGB8 = RGB8::new(100,0,0);
        const B: RGB8 = RGB8::new(0,0,0);
        const COLORS: [RGB8; 64] = [
            B,R,R,B,B,R,R,B,
            R,B,B,R,R,B,B,R,
            R,B,B,B,B,B,B,R,
            R,B,B,B,B,B,B,R,
            B,R,B,B,B,B,R,B,
            B,B,R,B,B,R,B,B,
            B,B,B,R,R,B,B,B,
            B,B,B,B,B,B,B,B,
        ];

        // Contruct signal
        const SIGNAL_LEN: usize = COLORS.len() * 24;  // 24 bits per LED
        let mut signal = FixedLengthSignal::<SIGNAL_LEN>::new();
        let p = &self.pulse; // shortcut
        for (j, rgb) in COLORS.iter().enumerate() {
            // Color: convert to GRB encoding
            let color: u32 = ((rgb.g as u32) << 16) | ((rgb.r as u32) << 8) | rgb.b as u32;

            // Prepare the lengths: 24 bits of data, encoded.
            for i in 0..24 {
                // Get the bit
                let bit = (color & 1<<(23-i)) != 0;
                // Convert to a pair of Pulses: level + duration
                let (high_pulse, low_pulse) = if bit { (p.t1h, p.t1l) } else { (p.t0h, p.t0l) };
                // Add to signal
                signal.set(j*24 + i as usize, &(high_pulse, low_pulse))?;
            }
        }

        // Send the signal
        self.rmt.start_blocking(&signal)?;

        // Done
        Ok(())
    }
}

fn ns(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}

```



# tutorials/a04-std-idf-http-client/src/blinky_led.rs

```rust
use std::time::Duration;

use esp_idf_hal::{
    delay::FreeRtos, gpio::{OutputMode, Pin, PinDriver, Level}
};
use anyhow::{Result};

// Blinky LED controller.
// It's a generic because every PIN is a different type :O
pub struct BlinkyLed<'d, P: Pin, M: OutputMode> {
    output: PinDriver<'d, P, M>,
    inverted: bool
}

impl <'d, P: Pin, M: OutputMode> BlinkyLed<'d, P, M> {
    pub fn new(
        mut output: PinDriver<'d, P, M>,
    ) -> Result<Self> {
        output.set_level(Level::Low);
        Ok(Self{
            output,
            inverted: false,
        })
    }

    pub fn new_inverted(
        mut output: PinDriver<'d, P, M>,
    ) -> Result<Self> {
        output.set_level(Level::High);
        Ok(Self{
            output,
            inverted: true,
        })
    }

    pub fn blink(&mut self, n: u8, on: Duration, off: Duration) -> Result<()>{
        for _ in 0..n {
            self.output.set_level(if self.inverted { Level::Low } else { Level::High });
            FreeRtos::delay_ms(on.as_millis() as u32);

            self.output.set_level(if self.inverted { Level::High } else { Level::Low });
            FreeRtos::delay_ms(off.as_millis() as u32);
        }
        Ok(())
    }
}
```



# tutorials/a04-std-idf-http-client/src/wifi.rs

```rust
use anyhow::Result;
use log;

// IDF
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::peripheral,
    wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};

// Create a new WiFi connection.
// It also binds
pub fn new<'a>(
    ssid: &str,
    pass: &str,
    modem: impl peripheral::Peripheral<P=esp_idf_svc::hal::modem::Modem> + 'a,
    sysloop: EspSystemEventLoop,
    // Result will live as long as `modem` lives
) -> Result<EspWifi<'a>> {
    // Init WiFi. Bind network to it.
    // This binds the IDF implementation of OSI Layer 2 (the "Data Link" layer that sends/receives ethernet packets)
    // to standard Rust networking: as a result, you can use network functions.
    // Also, we wrap it into a strictly blocking wrapper: internally, it waits for events (using the event loop)
    // and keeps polling is_connected(), etc.
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    // Start WiFi and scan for networks.
    // We start with blank configuration. Will set the SSID later.
    log::info!("Starting wifi...");
    wifi.set_configuration(&Configuration::Client(ClientConfiguration{
        // Fast scan. Find one SSID only.
        // ssid: ssid.try_into().unwrap(),
        // scan_method: esp_idf_svc::wifi::ScanMethod::FastScan,
        ..Default::default()
    }))?;
    wifi.start()?;
    log::info!("Scanning...");
    let ap_infos = wifi.scan()?;

    // Find our network: using SSID match.
    // If found, read its channel & auth method.
    // If not, we'll proceed with unknown channel
    let found_ap = ap_infos.into_iter().find(|a| a.ssid == ssid);
    let (channel, auth_method) = if let Some(ap) = found_ap {
        // let formattedBssid = {
        //     let [a, b, c, d, e, f] = ap.bssid;
        //     format!("{a:02X}:{b:02X}:{c:02X}:{d:02X}:{e:02X}:{f:02X}")
        // };
        // log::info!("AP found: {ssid} on channel {} signal={} auth={:?} bssid={}",
        //     ap.channel, ap.signal_strength, ap.auth_method, formattedBssid);
        (Some(ap.channel), ap.auth_method)
    } else {
        log::info!("AP not found: {ssid}. Will go with defaults.");
        (None, None)
    };

    // Set configuration now and connect.
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),    // converts into heapless::String
        password: pass.try_into().unwrap(),
        // Use auth method from the AP.
        // If not found, use None (if no password) or WPA2 (good default)
        auth_method: auth_method.unwrap_or({
            if pass.is_empty() { AuthMethod::None } else { AuthMethod::WPA2Personal }
        }),
        channel,
        //  ESP-IDF tries to register the hostname "espressif" in your local network,
        // so often http://espressif/ will work. You can customize the hostname using `sdkconfig.defaults`:
        // CONFIG_LWIP_LOCAL_HOSTNAME="esp32c3"
        // Docs: https://docs.espressif.com/projects/esp-idf/en/v5.5.1/esp32c3/api-reference/kconfig-reference.html
        ..Default::default()
    }))?;

    // Connect
    log::info!("Connecting wifi...");
    wifi.connect()?;
    log::info!("Waiting for DHCP lease...");
    wifi.wait_netif_up()?;
    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    log::info!("Got DHCP: {:?}", ip_info);

    // Done
    Ok(esp_wifi)
    // Ok(Box::new(esp_wifi))
}

```



# tutorials/a04-std-idf-http-client/src/http_client.rs

```rust
use anyhow::{Result, bail};

use esp_idf_svc::{
    // Manages HTTP connections.
    // See: embedded_svc::http::client::Client
    http::client::{EspHttpConnection, Configuration},
};
use embedded_svc::http::{Headers, Method, client::Client};

pub fn get_page(url: &str) -> Result<(u64, String)> {
    // Create a new ESP Http Connection.
    // Wrap it with embedded_svc's Client.
    let connection = EspHttpConnection::new(&Configuration{
        // By default, only unencrypted HTTP is available.
        // Here we enable the use of certificates:
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;
    let mut client = Client::wrap(connection);

    // Make a request
    let request = client.request(Method::Get, url, &[("accept", "text/plain")])?;
    let response = request.submit()?;

    // Status code
    let status = response.status();
    let content_len = response.content_len().unwrap_or(0);
    println!("Response code: {}\n", status);
    log::info!("Content-length: {content_len}");

    // Bad code?
    if !(200..299).contains(&status) {
        bail!("Bad status code: {status}");
    }

    // Good code.
    // Read response data into a buffer.
    let mut buf = [0_u8; 256];
    let mut offset = 0;
    let mut total = 0;
    let mut reader = response;
    loop {
        match reader.read(&mut buf[offset..]) {
            // EOF: size = 0
            Ok(0) => break,
            // Read N bytes
            Ok(n) => {
                offset += n;
                total += n;
                if offset >= buf.len() {
                    break; // buffer full
                }
            }
            // Fail
            Err(e) => return Err(e.into()),
        }
    }

    // Convert into UTF-8 Rust string.
    // NOTE: there's no guarantee that we ended up with a valid UTF-8 sequence: in fact, it is likely
    // that our chunked reader has broken up some sequences.
    // This edge case has to be handled.
    match str::from_utf8(&buf[..total]) {
        Ok(text) => Ok((content_len, text.to_string())),  //-> String
        Err(err) => {
            // The buffer has Incomplete UTF-8 data. Let's extract the valid part.
            let valid_up_to = err.valid_up_to();
            Ok((
                content_len,
                unsafe {
                    str::from_utf8_unchecked(&buf[..valid_up_to]).to_string()
                }
            ))
        }
    }
}
```





# tutorials/a05-std-idf-http-server/src


# tutorials/a05-std-idf-http-server/src/main.rs

```rust
// Cargo add:
// $ cargo add toml-cfg anyhow esp-idf-hal
use anyhow::Result;
use std::time::Duration;

// IDF HAL and services
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{self, OutputMode, PinDriver},
    prelude::*,
    temp_sensor
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    nvs::EspDefaultNvsPartition,
};

// Our libraries
use a04_std_idf_http_client::{
    blinky_led,
    wifi,
};

// Drivers initialized on peripherals
// Lifetime 'd is the "device lifetime" (conventionally)
struct Drivers<'d,
    // It's nice to have all the pinout in one place, isn't it?
    StatusLedPin: gpio::Pin = gpio::Gpio8,
    RedLedPin:    gpio::Pin = gpio::Gpio0,
    GreenLedPin:  gpio::Pin = gpio::Gpio2,
    BlueLedPin:   gpio::Pin = gpio::Gpio1,
> {
    status_led: blinky_led::BlinkyLed<'d, StatusLedPin, gpio::Output>,
    modem:      esp_idf_hal::modem::Modem,
    internal_temp: esp_idf_hal::temp_sensor::TempSensorDriver<'d>,
    // NOTE: The three diodes have a common Anode (+), and their Cathodes (-) are connected to pins.
    // This is called Active-LOW or Inverted Logic: the GPIO pin is now acting as a current sink.
    // Therefore, set the pin to LOW for the current to flow into the pin sink.
    red_led:    blinky_led::BlinkyLed<'d, RedLedPin, gpio::Output>,
    green_led:  blinky_led::BlinkyLed<'d, GreenLedPin, gpio::Output>,
    blue_led:   blinky_led::BlinkyLed<'d, BlueLedPin, gpio::Output>,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let sysloop = EspSystemEventLoop::take()?;
    let _nvs = EspDefaultNvsPartition::take()?;

    // Peripherals
    let config = CONFIG;
    let mut drv = Drivers::new()?;

    let temp = drv.internal_temp.get_celsius()?;
    log::info!("Internal temp: {temp} °C");

    // WiFi
    // TODO: move into `drv`
    log::info!("Board starting...");
    drv.status_led.blink(1, Duration::from_millis(100), Duration::from_millis(200))?;
    let _wifi = wifi::new(config.wifi_ssid, config.wifi_psk, drv.modem, sysloop)?;
    drv.status_led.blink(3, Duration::from_millis(100), Duration::from_millis(200))?;

    // Init HTTP server
    let _server = a05_std_idf_http_server::http_server::new(
        drv.internal_temp,
        drv.red_led,
        drv.green_led,
        drv.blue_led,
    )?;
    println!("Server awaiting connection");

    loop {
        FreeRtos::delay_ms(100);
    }
}


impl <'d> Drivers<'d> {
    fn new() -> Result<Self> {
        // Take the peripherals.
        // Note that the value is moved, and it was a singleton.
        // Now no one else can use the peripherals: only through us.
        let peripherals = esp_idf_hal::peripherals::Peripherals::take()?;

        // Configure
        let mut status_led_output = PinDriver::output(peripherals.pins.gpio8)?;
        let mut red_led_output = PinDriver::output(peripherals.pins.gpio0)?;
        let mut green_led_output = PinDriver::output(peripherals.pins.gpio2)?;
        let mut blue_led_output = PinDriver::output(peripherals.pins.gpio1)?;
        let mut internal_temp_sensor = temp_sensor::TempSensorDriver::new(&temp_sensor::TempSensorConfig::default(), peripherals.temp_sensor)?;

        // Drivers
        Ok(Self {
            status_led: blinky_led::BlinkyLed::new(status_led_output)?,
            modem: peripherals.modem,
            red_led: blinky_led::BlinkyLed::new_inverted(red_led_output)?,
            green_led: blinky_led::BlinkyLed::new_inverted(green_led_output)?,
            blue_led: blinky_led::BlinkyLed::new_inverted(blue_led_output)?,
            internal_temp: {
                internal_temp_sensor.enable()?;
                internal_temp_sensor
            },
        })
    }
}


#[toml_cfg::toml_config]
pub struct Config { // cfg.toml
    #[default("Wokwi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

```



# tutorials/a05-std-idf-http-server/src/lib.rs

```rust
pub mod http_server;

```



# tutorials/a05-std-idf-http-server/src/http_server.rs

```rust
use a04_std_idf_http_client::blinky_led;
use anyhow::Result;
use std::{
    sync::{Arc, Mutex}, time::Duration,
};
use esp_idf_hal::{
    io::EspIOError,
    gpio,
};
use esp_idf_svc::{
    http::server::{EspHttpServer, Configuration, Method},
};


// See this moved value, `temp_sensor`?
// Problem: it only lives for the duration of the function: i.e. it gets dropped when func quits.
//  But we have a closure that uses the value. Moreover, it should own it:
//  to be able to use it multiple times; it shouldn't become `FnOnce` either.
// Solution: use `Arc` (Atomic Reference Count) for shared ownership, and
//  `Mutex` for safe, exclusive access to the hardware driver.
// We could also mark `temp_sensor` and the returned `server` with 'a lifetime —
//  but the server's closure is required to be static: `Fn + 'static`.
//  Therefore, all values the static closure uses are also supposed to be static.
pub fn new<RedLedPin, GreenLedPin, BlueLedPin>(
    temp_sensor: esp_idf_hal::temp_sensor::TempSensorDriver<'static>,
    red_led: blinky_led::BlinkyLed<'static, RedLedPin, gpio::Output>,
    green_led: blinky_led::BlinkyLed<'static, GreenLedPin, gpio::Output>,
    blue_led: blinky_led::BlinkyLed<'static, BlueLedPin, gpio::Output>,
) -> Result<EspHttpServer<'static>>
    where
        RedLedPin:    gpio::Pin,
        GreenLedPin:  gpio::Pin,
        BlueLedPin:   gpio::Pin,
{
    // Wrap the owned driver in a `Mutex` for safe mutable access, and an `Arc` for shared ownership.
    let shared_sensor = Arc::new(Mutex::new(temp_sensor));

    // Wrap the leds
    let red_led = Arc::new(Mutex::new(red_led));
    let green_led = Arc::new(Mutex::new(green_led));
    let blue_led = Arc::new(Mutex::new(blue_led));

    // Init the HTTP server
    let mut server = EspHttpServer::new(&Configuration{
        http_port: 80,
        ..Default::default()
    })?;
    server.fn_handler("/", Method::Get, |request| -> core::result::Result<(), EspIOError> {
        // Show index page
        let html = index_html_templated("Hello from ESP32!");
        let mut response = request.into_ok_response()?;
        response.write(html.as_bytes())?;
        Ok(())
    })?;

    // Callback uses the value. It's moved.
    server.fn_handler("/temperature", Method::Get, move |request| -> core::result::Result<(), EspIOError> {
        // Inside the closure, lock the Mutex to gain exclusive access to the driver.
        // It will get unlocked when the function quits: with .drop()
        let sensor = shared_sensor.lock().unwrap();
        let temp = sensor.get_celsius()?;

        // Show temperature page
        let html = index_html_templated(format!("Chip temperature: {:.2}°C", temp));
        let mut response = request.into_ok_response()?;
        response.write(html.as_bytes())?;
        Ok(())
    })?;

    server.fn_handler("/blink", Method::Get, move |request| -> core::result::Result<(), EspIOError> {
        // Blink
        let uri = request.uri();
        if uri.contains("?blink=red") {
            red_led.lock().unwrap().blink(3, Duration::from_millis(100), Duration::from_millis(100));
        }
        if uri.contains("?blink=green") {
            green_led.lock().unwrap().blink(3, Duration::from_millis(100), Duration::from_millis(100));
        }
        if uri.contains("?blink=blue") {
            blue_led.lock().unwrap().blink(3, Duration::from_millis(100), Duration::from_millis(100));
        }

        // Show page
        let html = index_html_templated(r#"
        <ul>
            <li><a href="?blink=red">Red</a></li>
            <li><a href="?blink=green">Green</a></li>
            <li><a href="?blink=blue">Blue</a></li>
        </ul>
        "#);
        let mut response = request.into_ok_response()?;
        response.write(html.as_bytes())?;
        Ok(())
    })?;


    // We need to return the server so that someone owns it.
    Ok(server)
}

fn index_html_templated(content: impl AsRef<str>) -> String {
    format!(r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>
        /* Base styles */
        body {{
            font-size: 12pt;
        }}

        /* Smaller tablets and mobiles */
        @media (max-width: 768px) {{
            body {{
                font-size: 16pt;
            }}
        }}
        </style>
    </head>
    <body>
        {}
    </body>
</html>
    "#, content.as_ref())
}

```





# tutorials/a06-std-idf-mqtt/src


# tutorials/a06-std-idf-mqtt/src/main.rs

```rust
use anyhow::Result;

// Our libraries
use a04_std_idf_http_client::{
    blinky_led,
    wifi,
};

// IDF
use esp_idf_hal::{
    prelude::*,
    temp_sensor,
    // delay::FreeRtos,
};
use esp_idf_svc::{
    mqtt::client::{EspMqttClient, Details, MqttClientConfiguration},
};



// Drivers
struct Drivers<'d,
    // It's nice to have all the pinout in one place, isn't it?
    StatusLedPin: gpio::Pin = gpio::Gpio8,
> {
    status_led: blinky_led::BlinkyLed<'d, StatusLedPin, gpio::Output>,
    internal_temp: esp_idf_hal::temp_sensor::TempSensorDriver<'d>,
}


fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let sysloop = EspSystemEventLoop::take()?;
    let _nvs = EspDefaultNvsPartition::take()?;

    // Peripherals
    let config = CONFIG;
    let mut drv = Drivers::new()?;

    // WiFi
    let _wifi = wifi::new(config.wifi_ssid, config.wifi_psk, drv.modem, sysloop)?;
    drv.status_led.blink(3, Duration::from_millis(100), Duration::from_millis(200))?;

    // MQTT
    // NOTE: start a minimal MQTT server:
    //  $ docker run --rm -it -v ./nats-server.conf:/nats-server.conf -p 1883:1883 nats
    let mqtt_config = MqttClientConfiguration::default();
    let mut client = EspMqttClient::new_cb(&broker_url, &mqtt_config, move |message| {
        // MQTT messages receiver
        match message.payload() {
            Received { data, details, .. } => process_message(data, details, &mut led),
            Error(e) => warn!("Received error from MQTT: {:?}", e),
            _ => info!("Received from MQTT: {:?}", message_event.payload()),
        }
    })?;

    // Publish a message
    let payload: &[u8] = &[];
    client.enqueue(&hello_topic(UUID), QoS::AtLeastOnce, true, payload)?;

    loop {
        sleep(Duration::from_secs(1));
        let temp = temp_sensor
            .measure_temperature(PowerMode::NormalMode, &mut delay)
            .unwrap()
            .as_degrees_celsius();
        // 3. publish CPU temperature
        client.enqueue(
            &mqtt_messages::temperature_data_topic(UUID),
            QoS::AtLeastOnce,
            false,
            &temp.to_be_bytes() as &[u8],
        )?;
    }

    loop {
        FreeRtos::delay_ms(100);
    }
}

fn process_message(data: &[u8], details: Details, led: &mut WS2812RMT) {
    match details {
        Complete => {
            info!("{:?}", data);
            let message_data: &[u8] = data;
            if let Ok(ColorData::BoardLed(color)) = ColorData::try_from(message_data) {
                info!("{}", color);
                if let Err(e) = led.set_pixel(color) {
                    error!("Could not set board LED: {:?}", e)
                };
            }
        }
        _ => {}
    }
}

impl <'d> Drivers<'d> {
    fn new() -> Result<Self> {
        // Take the peripherals.
        // Note that the value is moved, and it was a singleton.
        // Now no one else can use the peripherals: only through us.
        let peripherals = esp_idf_hal::peripherals::Peripherals::take()?;

        // Drivers
        Ok(Self {
            status_led: {
                let mut status_led_output = PinDriver::output(peripherals.pins.gpio8)?;
                blinky_led::BlinkyLed::new(status_led_output)?
            },
            internal_temp: {
                let mut sensor = temp_sensor::TempSensorDriver::new(&temp_sensor::TempSensorConfig::default(), peripherals.temp_sensor)?;
                sensor.enable()?;
                sensor
            },
        })
    }
}


#[toml_cfg::toml_config]
pub struct Config { // cfg.toml
    #[default("Wokwi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
    #[default("")]
    mqtt_broker_url: &'static str,
}


```





# tutorials/b01-led-pwm/src/bin


# tutorials/b01-led-pwm/src/bin/main.rs

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
```





# tutorials/b02-led-embassy/src/bin


# tutorials/b02-led-embassy/src/bin/main.rs

```rust
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

```

