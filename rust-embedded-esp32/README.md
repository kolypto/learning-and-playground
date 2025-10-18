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

Lessons:

* 🇷🇺 [AlexGyver](http://alexgyver.ru/)
* 🇷🇺 [NarodStream](https://narodstream.ru/programmirovanie-esp32/)
* [ControllerTech](https://controllerstech.com/)

OSes and Frameworks:

* [Embassy: modern embedded framework](https://github.com/embassy-rs/embassy): async/await. HAL with batteries included. Clean, modern async code flow, great for I/O heavy or complex state machines. High productivity. Use case: IoT, network services.
* [Ariel OS](https://github.com/ariel-os/ariel-os): async/await, builds on top of Embassy.
  It's a LibOS: a library that brings together crates and combines them conveniently.
  Portable peripheral API: ESP, nRF, PI, STM.
  Preemptive Priority Scheduling.
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


## Memory

ESP provides `esp-alloc`: a `no_std` heap manager providing `Box`, `Rc`, `RefCell`, `Arc`.

When you init `esp-alloc`, you'd pre-configure it with a memory region that you're going to use.

```rust
// Init ESP heap allocator.
// Use the "reclaimed" region.
esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 64 * 1024);

// Init in the default RAM
esp_alloc::heap_allocator!(size: 72 * 1024);

// You can use DRAM2 (if your board has it)
esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);
```

ESP32 has fragmented RAM regions. Here we are manually carving out chunks for the allocator.

Multiple separate heap allocators:
- `reclaimed`: reclaimed RAM: memory freed after boot (was used by bootloader).
- default DRAM
- `.dram2_uninit`: uninitialized DRAM (section of Data RAM that is not initialized at boot).
  Some chips don't have it.

Other available regions: `rtc_fast`, `rtc_slow`, `persistent`, `zeroed`.

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






# topics
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
5. Switching regulator: doesn't heat up. Can increase or decrease voltage.
6. Level shifter, translator: converts digital signals from one logic standard to another.
   It's low power: i.e. it isn't meant to provide power
7. Linear regulator: can only be used to produce a lower voltage from a higher one

Chips for voltage regulation / level shifting:

* VHCT125A.  It's a 4-bus buffer, but can act as a level converter.
* AMS1117. A low-dropout voltage regulator: 5V to 3.3V linear regulator

### Voltage Divider

The HT-SR04 Ultrasonic Sensor is powered by 5V, which is fine.
But its output pin (echo) produces 5V as well, which ESP32 can't handle:
ESP32 is only around 3.6V tolerant on its GPIO pins.

Therefore we connect the "echo" output to GPIO *via voltage divider*:

<img src="https://esp32.implrust.com/ultrasonic/images/ESP32-HC-SR04-circuit.png">

```
                     ┌─────────── GPIO
 5V output ───[ 1k ]─┴──[ 2k ]─── GND
```

# Components

## Buzzer

There are two kinds of buzzers:

1. Active buzzer: have an internal oscillator => produce fixed frequency sound.
2. Passive buzzer: just a speaker.

How to identify:

* Active buzzer: apply constant voltage => produces sound
* Passive buzzer: apply voltage — will just click, but not buzz.
* Active buzzers may have a third pin: that's transistor base
* My buzzer was 3.3V

## LDR: Light-Dependent Resistor

Photoresistor: changes its resistance based on the amount of light falling on it.
The brighter the light, the lower the resistance, and the dimmer the light, the higher the resistance.

Mnemonic: Dracula. In sunlight he gets weaker :D But in the dark, he gets stronger.

You would typically connect it to one of the ADC pins (analog input) through a voltage divider.
Take a 10K resistor or larger, connect the GPIO pin in-between:

```
                        ┌──────────── GPIO
 3.3V output ───[ 10k ]─┴──[ LDR ]─── GND
```

The higher its resistance is, the higher the voltage: it produces a voltage drop
between the GPIO (central point) and the GND. The 3.3V gets dropped in proportion
to the ration between the two resistors.

## Thermistor

Changes its resistance based on temperature.
All resistors change with temperature, but thermistors do this in a predictable manner.

Thermistors are categorized into two types:

* NTC (Negative Temperature Coefficient): Resistance decreases as temperature rises.
* PTC (Positive Temperature Coefficient): Resistance increases as temperature rises.

NTSs are primarily used for temperature sensing and inrush current limiting.
PTCs primarily protect against overcurrent and overtemperature conditions as resettable fuses
and are commonly used in air conditioners, medical devices, battery chargers, and welding equipment.

Connection: voltage divider.
Read voltage with ADC.

Example: NTC 103 Thermistor: 10K OHM. Use another 10K resistor.

### Thermistor Equation

To convert a thermistor's resistance to temperature, use:

* the Steinhart-Hart equation: more accurate.
  You'll need to know `A`, `B`, `C` constants from the datasheet
  that are specific to the thermistor's material
* the B equation: simpler but less precise.
  You'll need to know the `B`-value, the `T0` and `R0` from the datasheet.

The B-equation:

  $1/T = 1/T_0 + (1/B) * ln(R/R_0)$

* $T$: temperature in K (Kelvin)
* $T_0$: thermistor's reference temperature; usually 25℃
* $R$: thermistor's measured resistance
* $R_0$: thermistor's reference resistance at $T_0$; often $10 K\Omega$
* $B$ is the *B-value* of the thermistor: constant based on its material. Typically $3950$

You can determine the constants by making a series of experiments: room temperature, ice water, boiling water.
See: [Calculating Steinhart-Hart Coefficients](https://esp32.implrust.com/thermistor/steinhart.html)


### Math

The voltage divider: either R1 or R2 can be the thermistor. No big deal.

```
                        ┌──────────────── GPIO4
  V_DD ──────[ R_1 ]────┴───[ R_2 ]────── GND
```

The voltage divider formula:

  $V_{out} = V_{DD} * R_2 / (R_1 + R_2)$

To solve for R:

  $R_1 = R_2 * (V_{DD}/V_{out} - 1)$

  $R_2 = R_1 / (V_{DD}/V_{out} - 1)$

The ADC, however, measures voltage according to its reference value, $V_{ref} = 1100 mV$:

  $V_{measured} = V_{ref} * {adc\_value} / {ADC\_MAX}$

with attenuation, e.g. $dB=11$:

  $V_{measured} = V_{ref} * 10^{dB/20} * {adc\_value} / {ADC\_MAX}$

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

sometimes in the "pinout" image you'll see GPIx: "x" replacing the "O" means the pin can't be used as an output.

## Drive Mode: Push/Pull vs Open Drain

Push-pull GPIOs actively drive a pin both HIGH and LOW:
i.e. there will be voltage when HIGH and it will be grounded when LOW.

Open drain pins can only actively drive low, requiring an external pull-up resistor for a high state.
That is, you use open drain only when you have an external pullup.

* Pull-up resistor: GPIO is connected to a switch (buffer, transistor base, ...) and also to VCC through a resistor.
  This makes it an active-LOW.
* Pull-down resistor: GPIO is connected to a switch (buffer, transistor base, ...) and also to GND through a resistor.
  This makes it an active-HIGH.


Open-drain: pin either disconnected and floating, or active low. There is no high. This has a consequence of the pin high voltage not being set at all, it is set externally through pull-up resistors. Which means you can set high voltage to pretty much any voltage within pin spec, but you can’t drive anything directly from the pin.

Open-drain is usually used with buses: e.g. I2C required it.


# WiFi

The ESP32's WiFi can operate in two modes:

* Station (STA) mode: WiFi client, connects to an AP
* Soft Access Point mode: WiFi Access Point

It is also capable of running in both modes simultaneously.

## Prepare

* Enable unstable hal
* Enable `esp-alloc`
* Enable `esp-radio`


# Embassy

## embassy-executor

`embassy-executor` is an async/await executor specifically designed for embedded systems.

It has a `nightly` Cargo feature:
when it's not enabled, it allocates tasks out of an arena (a very simple bump allocator).
If the task arena gets full, the program will panic at runtime.
To guarantee this doesn’t happen, you must set the size to the sum of sizes of all tasks.

The default arena size is 4096. However `esp-generate` uses `20480` as the default.

To configure the arena size:

* Use cargo feature: `task-arena-size-8192`
* Environment variables during build: `EMBASSY_EXECUTOR_TASK_ARENA_SIZE=8192 cargo build`.
  Environment variables take precedence over Cargo features.

When using `nightly` Rust, enable the `nightly` Cargo feature.
This will make `embassy-executor` use the `type_alias_impl_trait` feature to allocate all tasks in statics.
Each task gets its own `static`.
If tasks don’t fit in RAM, this is detected at compile time by the linker. Runtime panics due to running out of memory are not possible.

## `'static`

Embassy tasks need their arguments as `'static` values.
However, some peripherals that you init in `main()` aren't `'static`.

Here's how you can make them such:

### `static_cell::make_static!()`

Use `make_static!()` macro:

```rust
use static_cell::make_static;
let radio = make_static!(esp_radio::init().expect("Radio init"));
```

you can even choose which memory region to allocate it in:

```rust
let buf = make_static!([0u8; 4096], #[link_section = ".ext_ram.bss.buf"]);
```

However, in my case, the macro fails when ran inside `main()`.

### `static_cell::StaticCell`

You can initialize static values manually:

```rust
use static_cell::StaticCell;

static RADIO: StaticCell<esp_radio::Controller> = StaticCell::new();
let radio = RADIO.init(esp_radio::init().expect("Init radio"));
```

One issue: that value will be `&mut T`, and it will give exclusive access to just one task.
That's actually the whole point: otherwise multiple conflicting references would be possible!

To get rid:

```rust
&*radio
```

or just pass it to a func that accepts `&'static` argument: you'll lose the `&mut` part:

```rust
pub async fn run_tasks(bt_transport: &'static BleConnector<'static>) -> Result<()> {
    ...
}
```

Finally, here's a short version:

```rust
let radio = {
    static RADIO: StaticCell<esp_radio::Controller> = StaticCell::new();
    RADIO.init(esp_radio::init().expect("Init radio"))
};
```

### `mk_static!()`
If you don't want nightly Rust, use this macro:

```rust
#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}
```

### `once_cell`

```rust
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<Logger> = OnceCell::new();

fn setup() {
    INSTANCE.set(logger).unwrap();
    INSTANCE.get().expect("logger is not initialized"); // -> &T
}
```

### Mutex

Use this when multiple tasks need to co-operate:

```rust
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

static SHARED: Mutex<CriticalSectionRawMutex, Option<MyStruct>> = Mutex::new(None);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    *SHARED.lock().await = Some(MyStruct::new());
}
```


### `core::mem::MaybeUninit`

This is in `core` and bundled with Rust.
Note that `static_cell` already uses it under the hood :) So no point.

```rust
use core::mem::MaybeUninit;
static mut RADIO: MaybeUninit<esp_radio::Controller<'static>> = MaybeUninit::uninit();
let radio = unsafe {
    RADIO.write(esp_radio::init().expect("Init radio"));
    RADIO.assume_init_mut()
};
```


### Unsafe way

```rust
static mut BUFFER: [u8; 1024] = [0; 1024];

#[embassy_executor::task]
async fn my_task() {
    unsafe {
        BUFFER[0] = 42;
    }
}
```

# ADC: Analog-to-Digital Converter

ADC converts analog signals into digital.

## ADC Resolution

*ADC resolution* is the number of bits it uses to represent a reading:
it is also the precision:

* 8 bits: values between 0..255
* 10 bits: 0..1023
* 12 bits: 0..4096

In relation to a reference voltage:

> resolution = Vref / (2^bits - 1)

This means that for a 12-bit ADC and a 3.3V input, the resolution is 0.8 mV:

> 3.3V / (2^12−1) = 3.3V / 4095 = 0.8 mV

ESP32-C3 has two 12-bit SAR ADCs, up to 6 channels.
NOTE: ADC2 is not calibrated; also, on some chip revisions, it is not operable due to a bug!
Use ADC1 instead. (See: [errata](https://espressif.com/sites/default/files/documentation/esp32-c3_errata_en.pdf))

NOTE: The ADC in the ESP32 is known to have non-linearity issues.

## Pins

Not all pins are available for analog signal processing!
See "IO MUX" chapter.

In ESP32-C3:
* pins 0..4 are available to ADC1
* pin 5 is available to ADC2 (uncalibrated!)


## Reference voltage

The ADC needs a reference voltage to compare with the input voltage. This reference voltage,
called *V_ref* (Voltage Reference), helps the ADC map input voltages to digital values.

The ESP32 uses a $V_{ref} = 1100mV$.
This means it can only map input voltages between 0nV and 1100mV.

However, due to manufacturing variations, the actual value may range
between $1000 mV$ and $1200 mV$ depending on the chip
(see: [Reference: ADC](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-reference/peripherals/adc/index.html)). Calibration needed!

The SAR resolution is 12 bits: $2^{12} - 1 = 4095$.
The output value:

  $V_{data} = V_{ref} * {adc} / 4095$

## Attenuation

But what happens when the input voltage is higher than 1.1V?
Use programmable *attenuation*: it reduces input voltages to fit into the range.
Attenuation is configurable in the code, in dB.

dB (decibels) express ratios logarithmically:

  ${dB} = 20 × log₁₀(V_{out} / V_{in})$

However, the [Reference: ADC](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-reference/peripherals/adc/index.html)) gives this table:

* $db=0$, range = 0 mV ~ 750 mV
* $dB=2.5$, range = 0 mV ~ 1050 mV
* $dB=6$, range = 0 mV ~ 1300 mV
* $dB=11$, range = 0 mV ~ 2500 mV

And this is how you find the actual reading:

  $V_{measured} = V_{max} * {adc\_value}/{ADC\_MAX}$

Mind the ADC non-linearity at the extremes.
Near 0V and near max, the ADC readings become inaccurate and noisy.
Therefore keep your range in the middle.

TODO: read about ADC calibration!
<https://docs.espressif.com/projects/rust/esp-hal/latest/esp32c3/esp_hal/analog/adc/index.html>
Uncalibrated ADC can give 30% errors and more!


## Example: Read Voltage from GPIO

```rust
// ADC Input
// Attenuation: -11dB
let ldr_pin = peripherals.GPIO0;
let mut adc_config = adc::AdcConfig::new();
let mut pin = adc_config.enable_pin(ldr_pin, adc::Attenuation::_11dB);
let mut adc1 = adc::Adc::new(peripherals.ADC1, adc_config);

// Value: 0..4095
let value: u16 = match nb::block!(adc1.read_oneshot(&mut pin)) {
    Ok(result) => result,
    Err(err) => {
        log::error!("err={err:?}");
        continue;
    }
};
```

## Example: Read Voltage from
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

SPI: Serial Peripheral Interface.

* Master-slave architecture
* Synchronous: clocked
* Serial: transfers bits one by one rather than in parallel

SPI is de-factor standard for synchronous serial communication in embedded systems.
Originally invented by Motorola in ~1980s.
Allows interfacing with peripheral chips, LCD displays, ADC/DAC converters, flash, EEPROM, and other chips.

SPI follows a master–slave architecture, where a master device orchestrates communication
with one or more slave devices by driving the clock and chip select signals.

It uses 4 wires to support full duplex (FDX).
In contrast to 3-wire variants which are half-duples (HDX): one direction at a time.

* One unique benefit of SPI is the fact that data can be transferred without interruption:
  no start/stop bits and no taking turns.
* SPI provides higher throughput than I²C or SMBus, uses less power,
  but it requires more pins.
* No complicated slave addressing system like I2C
* Higher data transfer rate than I2C: almost twice as fast
* However, no acknowledgment that the data has been successfully received

As long as you have enough GPIO pins and don't need to connect a large number of devices,
SPI is usually the best tool for the job.


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

Alternative pin naming:

* **CS**, **SCL**, **SDI** (Serial Data Input) and **SDO** (Serial Data Output)
* **SPIQ**, **SPID**: MISO / MOSI nicknames
* **SDA** can reuse the name from I2C.
* **SDIO**: 3-wire SPI (Serial Data Input/Output)
* **FSPI**: fast API

## Chip Select

How chip-select works: when the *chip select* pin is held in the inactive state (HIGH),
the chip remains "deaf" and pays no heed to changes in the state of its other input pins:
it holds its outputs in the *high impedance state* (Hi-Z, electrically disconnected),
so other chips can drive those signals.

When the chip select pin is held in the active state (LOW), the chip or device assumes that
any input changes it "hears" are meant for it and responds as if it is the only chip on the bus.

To begin communication, the SPI master first selects a slave device by pulling its S̅S̅ LOW.
(The bar above S̅S̅ indicates it is an active LOW signal, so a LOW voltage means "selected",
while a HIGH voltage means "not selected")

Caveat: All S̅S̅ signals should start HIGH before sending initialization messages to any slave.
Either configure your S̅S̅ GPIOs to be initially HIGH, or add a pull-up resistor on each S̅S̅,
to ensure that all S̅S̅ signals are initially high.

The max number of slaves is theoretically unlimited,
but in practice is limited by the load capacitance of the system: high-capacitance wires
would fail to switch between voltage levels.

## Data Transfer

Each device internally uses a [shift register](https://en.wikipedia.org/wiki/Shift_register) for serial communication,
which together forms an inter-chip circular buffer. (A *shift register* is a cascade of flip-flops:
latches with two stable states that can store a bit of information. The cascade shares the clock signal,
which causes the data stored in the system to shift from one location to the next.
By connecting the last flip-flop back to the first, the data can cycle in the register.)

Data is usually shifted out with the most-significant bit (MSB) first.

## Clock

The speed of data transfer is determined by the frequency of the clock signal.

During each SPI clock cycle, full-duplex transmission of a single bit occurs.
The master sends a bit on the MOSI line while the slave sends a bit on the MISO line,
and then each reads their corresponding incoming bit. This sequence is maintained even when
only one-directional data transfer is intended.

The Master must also configure the *clock polarity and phase* with respect to the data.
Motorola called this CPOL and CPHA (Clock POLarity and Clock PHAse).

Two options for CPOL:

* `CPOL=0`: a clock where idle = logical LOW.
* `CPOL=1`: a clock where idle = logical HIGH.

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

Mode 0 is the most common and works with most devices.

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
  Devices must support this mode explicitly
* Expander configurations: use SPI-controlled addressing units to add chip selects
  using demultiplexers.


## Variations

* Extra timing: Devices often require extra clock idle time: before the first clock, or after the last one, or between a command and its response.
* Dual SPI, Quad SPI: use additional data lines to transfer more bits at a time
* DDR: Transfer 2 bits per clock cycle


## ESP32

ESP32-C3 has 3 SPI peripherals.
Only SPI2 is available for general use.

ESP32-C3 does not have on-chip memory and relies on an off-chip SPI flash.
SPI0/SPI1 pins for flash connection are not bonded for variants with 16 GPIOs.

By default `VDD_SPI` is the power supply pin for in-package and off-package flash.
It can be reconfigured as a GPIO pin.


## Rust

Two important traits for SPI are:

* `SpiBus`: Represents full control over the SPI bus, including the `SCK`, `MOSI`, and `MISO` lines.
  This must be implemented by the microcontroller's HAL crate: for example, `esp-hal` crate implements `SpiBus`.
* `SpiDevice`: Represents access to a single SPI device that may share the bus with others.
  It takes control of the chip select (CS) pin and ensures the device is properly selected before communication and released afterward.

The `embedded-hal-bus` crate provides ready-to-use wrappers that implement the SpiDevice trait for you.

If your project only uses one SPI device and doesn't need sharing, you can use the `ExclusiveDevice` struct.
But if your project has multiple SPI devices sharing the same bus, choose:

* `AtomicDevice`
* `CriticalSectionDevice`





## Links

* [CircuitBasics: Basics of API Communication Protocol](https://www.circuitbasics.com/basics-of-the-spi-communication-protocol)
* [SparkFun: SPI](https://learn.sparkfun.com/tutorials/serial-peripheral-interface-spi/all)
* [TI: SPI User Guide](https://www.ti.com/lit/ug/sprugp2a/sprugp2a.pdf?ts=1762512235969)


# I2C: Inter-Integrated Circuit

I2C is:

* Synchronous: clocked, i.e. changes in the state of memory (flip-flops) are synchronized by a clock signal
* multi-master/multi-slave
* single-ended: one wire for reference voltage (GND) and another one for varying voltage; in contrast to differential signalling
* serial: one bit at a time; in contrast to parallel communication, where several bits are sent as a whole

Usage: attaching low-speed peripherals in short-distance, intra-board communication.

I2C bus was invented in 1980 by Philips Semiconductors (now NXP Semiconductors).

Features:

* Can control a network of device chips with just two GPIO pins

Variations:

* SMBus (commonly found on motherboards) is a stricter subset of I2C
* PMBus: power management bus
* Modern I2C systems incorporate some policies and rules from SMBus, sometimes supporting both

## Speed

Speeds:

* Standard mode: 100 kbits/s
* Fast mode: 400 kbits/s. Is highly compatible.
* Low-speed mode: 10 kbit/s
* Arbitrarily low clock frequencies are also allowed

Later revisions of I2C:

* Can host more nodes
* Can run at faster speeds:

  * Fast mode plus: 1 Mbit/s
  * High-speed mode: 3.4 Mbit/s
  * Ultra-fast mode: 5 MBit/s

The actual transfer rate is lower: because protocol overhead includes a target address, a register address, and per-byte ACK/NACK bits.

## Addressing

The number of nodes is limited by the address space, and also by the total bus capacitance of 400 pF.
This restricts practical communication distance to a few meters.
However, buffers can be used to isolate capacitance on one segment from another.

The bus has two roles:

* Master (controller): generates the clock and initiates communication
* Slace (target): receives the clock and responds when addressed by the controller

Address space: 7 bits, with a rarely used 10-bit extension.
This gives 112 addresses.

7-bit vs 8-bit confusion: Datasheets show addresses as 7-bit (`0x48`) or 8-bit with R/W bit included (`0x90`/`0x91`).
Rust I2C libraries want 7-bit. If you see a datasheet with `0x90`, shift right by 1:
`0x90 >> 1 = 0x48`.

Many devices have a fixed address defined by the manufacturer,
but others allow configuring the lower bits of the address using pins or jumpers.
This allows using multiple copies of the same chip on the same bus.

Reserved addresses:

* `0x00`: "General Call" address. For broadcasting.
  Example: a bus-wide reset, or driving multiple identical devies.
  Legacy feature, rarely used in modern designs.
* `0000 000 1`: Start byte
* `1111 1XX 1`: Device ID
* `1111 0XX X`: extension for 10-bit addressing
* Also: CBUS address, reserved, HS-mode controller code

## Modes of Operation

Modes of operation of a device:

* Master transmit
* Master receive
* Slave transmit
* Slave receive

Multiple masters can use the same bus by properly using
START and STOP messages (transactions): i.e. when one master
is done, another one acan use the bus.

## Pull-Up Resistors

Unlike UART or SPI connections, the I2C bus drivers are "open drain", meaning that they can pull the corresponding signal line low, but cannot drive it high.
Therefore, both SDA and SCL lines require pull-up resistors.

The value of pull-up resistors must be chosen to balance the rise time, power consumption, and signal integrity.

Start with 4.7K resistor ; adjust down if necessary (if the rise time is too slow).
For long buses / systems with lots of devices, smaller resistors are better.
Generally, 10K resistors work fine.

Note that many devices have pull-up resistors built into them.
If you have multiple decices on the same bus, you may need to remove some of those resistors.

### Rise Time and Capacitance
Rise time: the time it takes for a signal to transition from a low voltage level to a high voltage level.
The bus capacitance, or its ability to store a charge, affects rise time because it determines how quickly a circuit can charge.

Lower resistance values (Ex: 1 kΩ to 4.7 kΩ) allow more current to flow through the pull-up resistors. This increased current charges the capacitance of the bus lines more quickly, resulting in faster rise times which is vital in higher speed communications. While lower resistance values improve rise times, they can also lead to an excessive flow of current, causing unnecessary power consumption and potential damage to the components.

Higher resistance values (Ex: 10 kΩ) reduce the current flowing through the pull-up resistors, resulting in a slower charging of the bus line capacitance. This results in longer rise times, which can cause delays in signal transitions and potentially lead to bus errors or unreliable communication.

### Pull-Up Resistor Calculation

The spec suggests: calculate min resistance, max resistance, and choose one in-between.

Max resistance: based on bus capacitance and rise time. Measure the capacitance or estimate/simulate it.

Find the rise time on the device's datasheet. This is the time it takes to rise from V_IL(MAX)(0.3*VDD) to V_IH(MIN)(0.7*VDD)

Now:

> Rp(max) =  Trise/0.8473×Cbus, where 0.8473 is ln(0.7)-ln(0.3).

The min value is based on the supply voltage:
it must meet the minimum sink current requirements:

* 3 mA for Standard mode (100 kHz) or Fast mode (400 kHz)
* 20 mA for Fast mode Plus (1 MHz).

Now:

> Rp(min)= V_DD-V_OL(max)/I_OL

The trade-off between Rp(min) and Rp(max) is a tradeoff between speed and power:

* lower resistance values reduce power consumption and improve speed but increase current draw
* lower resistance improves the edges of signal transitions: they become more sharp
* higher values saves power, but may slow down signal transitions and lead to communication errors
* higher frequencies require lower resistance pull-ups: a lower resistance will charge/discharge the cable's capacitance faster.

Another formula [from the ATmega168 datasheet](https://electronics.stackexchange.com/a/1852/626574):

```
Freq < 100 kHz => Rmin = (Vcc - 0.4V) / 3ma , Rmax = 1000ns/Cbus
Freq > 100 kHz => Rmin = (Vcc - 0.4V) / 3ma , Rmax = 300ns/Cbus
```

The `Cbus` can be approximated: 10pF per pin × number of devices on the bus.


#### Save Power

Reality: I2C's open-drain design inherently wastes power.

Here's how to save power on the I2C bus if your device is battery-powered:

1. Use higher pull-ups. Works if your bus is short and slow (100 kHz).
2. Use lower bus voltage: 3.3V instead of 5V saves 33% power at same resistance.
3. Use internal pull-ups: many MCUs have configurable internal pull-ups (20-50KΩ).
   Weaker but draws less. Only works for very short buses.
4. Use SPI instead: SPI doesn't need pull-ups (push-pull outputs). More pins but lower idle power.
5. Disable pull-ups when idle: use GPIO+transistor to control pull-up power:

> MCU GPIO → transistor → pull-up resistors → I2C lines


## Implementation

I2C uses only two signals:

* SDA (or SDI): serial data line.
* SCL (or SCK): serial clock line. Generated by the master device.
* GND: common ground, of course

Typical voltages used are +5 V or +3.3 V.
Because devices don't actually drive the signals high, different voltages can co-exist on the same bus!
The trick is to connect the pull-up resistors to the lower of the two voltages.
But to be on the safe side, use logic converter or I2C level shifter.

A device pulls the SDA line low to transmit a bit. The line is high when idle, thanks to the pull-up resistor.
Communication is synchronized by the SCL line, with each bit transferred during each clock pulse.

A logic "0" is output by pulling the line to ground, and a logic "1" is output by letting the line float (output high impedance) so that the pull-up resistor pulls it high.
A line is never actively driven high. This wiring allows multiple nodes to connect to the bus without short circuits from signal contention:
i.e. multiple nodes may be driving the lines simultaneously.
If any node is driving the line low, it will be low.

NOTE: Because I2C is a shared bus, there is the potential for any device to have a fault and hang the entire bus.





### Clock Stretching

Slave devices can "stretch" the clock by holding the SCL line low to slow down communication if they need more processing time.
So, when someone pulls SCL low, it stretches the clock.
The controller will then have to wait until it goes high again before sending the next bit.

After the controller observes the SCL line going high,
it must wait an additional minimal time (4 μs for standard 100 kbit/s I2C) before pulling the clock low again.

### Arbitration
When someone pulls SDA low, this is called *arbitration*:
two controllers may start transmission at about the same time.

In contrast to Ethernet, whcih uses random back-off delays, I2C has a deterministic
arbitration policy to ensure one transmitter at a time.
Each transmitter checks the level of SDA and compares it with the level it expects.
If they do not match, that transmitter has lost arbitration and drops out.

The first node to notice such a difference is the one that loses arbitration.
In the meanwhile, another node sees no disturbance in the message and proceeds.

As with clock stretching, not all devices support arbitration.
Those that do, generally label themselves as supporting "multi-controller" communication.

### Sharing SCL
It is possible to have multiple I2C bses share the same SCL line.
The packets will be sent at the same time.

## Communication

In addition to 0 and 1 data bits, I2C allows special START and STOP signals that are distinct from the data bits.
They act as message delimiters.

* START: SDA goes high-to-low, with SCL high.
* STOP: SDA goes low-to-high, with SCL high.
* Data: all other transitions of SDA take place with SCL low.
* It is illegal, but harmless, to do multiple SDA transitions while SCL is high.

Sequence:

1. The master is initially in Master-Transmit mode.
2. It sends START + 7 bits of address of the target + 0 (write) or 1 (read)
3. The target responds with an ACK bit (active low).
4. Master continues in transmit or receive; target continues in the complementary mode: receive or transmit
5. After every 8 data bits in one direction, an "acknowledge" bit is transmitted in the other direction.
6. The controller terminates a message with a STOP condition; or it may send another START

Bit order: MSb (most significant bit first).

Pure I2C supports arbitrary message structures:
i.e. messages are product-specific.

SMBus is restricted to 9 specific commands — like "read word N" and "write word N".

### Combined Transaction

In addition to *single message* mode (read or write),
I2C defines a *combined transaction*: master issues multiple reads/writes to multiple targets.

* Each read/write begins with a START + target address.
* These *repeated START bits* are not preceded by STOP conditions:
  this is how targets know that the next message is part of the same transaction
* The terminating STOP indicates when those grouped actions should take effect

A combined transaction allows you to apply an operation atomically:
e.g. you can configure multiple paramenters on a power chip
and make sure the combined effect takes place.

Example: single transaction:

```
START → Address+Write → Data → STOP
START → Address+Read → Data → STOP
```

The problem is that another master may jump in between these two.

A combined transaction:

```
START → Address+Write → Data → RESTART → Address+Read → Data → STOP
```

When to use:

* Register read: write register address — then read its value without releasing the bus
* Multi-register operations:
Set multiple registers that must change together

When there's only one master on the bus, it doesn't matter.
But many I2C devices (example: EEPROM) require the *repeated START* pattern for register reads!
Example: An accelerometer expects you to send register address then immediately read without a STOP. The STOP would reset its read pointer.


## ESP32

The ESP32-C3 includes one I2C bus.
It supports up to 800 kbit/s, 7-bit and 10-bit addressing mode.
Only GPIO 8 and 9 can be used for I2C.

The ESP32-C6 has I2C controllers: one in the main system and one in the low-power system


## Rust

In Rust, `embedded-hal` (and `embedded-hal-async`) defines a trait.
Microcontroller-specific HAL crates (like `esp-hal`, `stm32-hal`, or `nrf-hal`) implement this trait.

Example:

```rust
// I2C. Init in async mode.
let i2c_bus = i2c::master::I2c::new(
    peripherals.I2C0,
    i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
).expect("Init I2C bus").with_sda(peripherals.GPIO8).with_scl(peripherals.GPIO9).into_async();
```

The `embedded-hal-bus` crate provides wrapper types like `AtomicDevice`, `CriticalSectionDevice`, and `RefCellDevice`
that allow multiple drivers to safely share access to the same I2C bus.
It basically wraps the `I2C` instance so that you can
share it across multiple drivers.



## Links

* [SparkFun: I2C](https://learn.sparkfun.com/tutorials/i2c/all)
* [TI: A Basic Guide to I2C](https://www.ti.com/lit/pdf/sbaa565)
* [CircuitBasics: Basics of the I2C Communication Protocol](https://www.circuitbasics.com/basics-of-the-i2c-communication-protocol/)
* [`embedded-hal/i2c`: in-depth details](https://docs.rs/embedded-hal/latest/embedded_hal/i2c/index.html)

# UART

UART: Universal Asynchronous Receiver-Transmitter.

* Asynchronous: not synchronized by a clock signal. Synchronization information is embedded in the data itself:
  start and stop signals set before and after each payload transmission.
* Data format and transmission speeds are configurable
* Sends data bits one by one, LSB-first.
* Data is framed by start and stop bits so that precise timing is handled by the communication channel.
* Only uses two wires
* Has error checking: 1 bit checksum

Connection: Tx to Rx, Rx to Tx.

## Baud Rate

There's no clock signal, but there's *baud rate*: speed of transfer, bps (bits per second).

In UART, both the transmitting and receiving devices must agree on the same baud rate to ensure successful communication.
The rx/tx baud rate can only differ by about 10%: otherwise, errors arise.


## The Protocol

Packet:

```
 -----------------------------------------------------------------------
|  1 start bit  |  5-9 data bits  |  0-1 parity bits  |  1-2 stop bits  |
 -----------------------------------------------------------------------
                   \ Data frame /
```

The UART line is normally held at HIGH voltage when idle.

Start bit: pulls the line LOW for one clock cycle.
This signals the receiver to start receiving.

Data frame: the payload. 5-8 bits + parity bit, or 9 bits with no parity bit.
Order: LSB-first.

Parity bit: the checksum. It's set to `0` when the number of `1`s is even, `1` if odd.

Stop bit: signals the end of the data packet.
The sender pulls the line to HIGH for at least 2 bit durations.


## Hardware

It's not just a protocol: it's a physical circuit in a microcontroller
that converts parallel data (e.g. a whole byte) into serial form: bit by bit.

Essentially, the UART is an intermediary between parallel and serial interfaces.
It connects a *data bus* to a remote end using just two wires.
Input: 8 data linse + CLK + INT + R/W; Output: Rx, Tx

Some microcontrollers may have multiple UART peripherals.

More advanced UARTs may throw their received data into a buffer, where it can stay until the microcontroller comes to get it.

Software emulation: if a microcontroller doesn't have a UART, the serial interface
can be *bit-banged*: i.e. directly controlled by the processor.
Note that bit-banging is CPU-intensive and not usually as precise.


## Flow Control

Flow Control is a mechanism where the receiver can tell the sender that it's overwhelmed
and cannot keep up.

* RTS/CTS flow control, Hardware flow control: uses 2 extra wires to signal the transmitter to stop & resume.
  They are called RTS (Request to Send) and CTS (Clear to Send).
  These wires are cross-coupled: RTS to CTS, CTS to RTS.
  Each device will use its RTS to output if it is ready to accept new data and read
  CTS to see if it is allowed to send data to the other device.
* Legacy hardware flow control: it also uses two wires, but it is unidirectional, with master/slave relationship.
  Connection is straight coupling: RTS-RTS, CTS-CTS.
  When the master wants to transmit data to the slave it asserts the
  RTS line. The slave responds by asserting CTS. Transmission can then occur until the slave deasserts CTS, indicating that it needs a
  temporary halt in transmission. When the master has finished transmitting the entire message it will deassert RTS.
* Software flow control: does not use extra wires. Instead, transmission is started and stopped by
  sending special flow control characters: typically, ASCII codes XON and XOFF (0x11 and 0x13).

More info: [UART Flow Control](https://www.silabs.com/documents/public/application-notes/an0059.0-uart-flow-control.pdf)

## Links

* [CircuitBasics: Basics of UART Communication](https://www.circuitbasics.com/basics-uart-communication/)
* [SparkFun: Serial Communication](https://learn.sparkfun.com/tutorials/serial-communication/uarts)
* [UART Flow Control](https://www.silabs.com/documents/public/application-notes/an0059.0-uart-flow-control.pdf)
# Bluetooth

Bluetooth categories:

* Bluetooth Classic: for devices that require continuous data transfer. Higher data rates.
  Previously called just "Bluetooth", now "Classic" — to distinguish it from BLE.
* Bluetooth Low Energy (BLE): designed for low power consumption.
  Takes less time to set up a connection. Ideal for IoT.

ESP32 supports *Dual-Mode Bluetooth*: i.e. both modes are available simultaneously.

## BLE Stack

* RADIO
* Controller layers:
  * PHY: Physical Layer
  * LL: Link Layer
  * Isochronous layer
* **HCI**: Host Controller Interface
* Host:
  * **L2CAP**: Logical Link Control & Adaptation Protocol
  * **SMP**: Security Manager
  * **ATT**: Attribute Protocol
  * **GATT**: Generic Attribute Profile
    How devices exchange and structure data
  * **GAP**: Generic Access Profile:
    How devices connect and communicate
* APP: Your application

GATT — how devices exchange and structure data.
Defines how BLE devices exchange data. It organizes data in a hierarchy of *services*
and characteristics, allowing clients to read, write, and subscribe to updates from a BLE peripheral.

GAP — how devices connect and communicate.
It covers device roles (e.g. central, peripheral), connection parameters, security modes.

### GAP

GAP: Generic Access Profile — discover, connect, communicate with devices.

BLE communication ways:

1. Connected communication: two devices, a direct connection, duplex data exchange
2. Broadcast: a bluetooth beacon continuously sending updates

Device roles:

* Broadcaster: the beacon
* Observer: beacon receiver
* Central: connected communication. Initiator.
* Peripheral: connected communication. Advertises iteslf, accepts connection.

BLE peripheral discovery modes:

1. Non-Discoverable: cannot be discovered or connected to.
   This is the default mode when a connection is established, or when no advertising is active.
2. Limited-Discoverable: discoverable for a limited time to save power.
3. General-Discoverable: advertises indefinitely until a connection is established

Advertisement flags:

* Discoverable: limited or general?
* Bluetooth Classic: possible?
* BLE + Classic: possible at the same time?


### ATT and GATT

After the GAP layer helps BLE devices find each other and connect, ATT and GATT layers
define how data is structured and transmitted between devices.

In the GATT, there are two roles:

* Server: holds data as attributes.
  E.g. a peripheral device (like a sensor) acts as a server.
* Client: accesses the server's data

Roles may swap: e.g. the smartphone (client) reads fitness data from a tracker (server),
but then the smartphone (server) needs to send configuration to the tracker (client).

ATT: Attributes. Defines how data is stored as attributes.
Each attribute has:
* a unique handle
* type (16-bit identifier or 128-bit UUID)
* permissions (readable, writable)
* data: the actual value.
The client can read, write, or subscribe to data.

Attributes are like remotely accessible registers: a software abstraction of those.
They are stored in RAM that the BLE stack exposes over the air. And unlike registers,
they are structured.

GATT adds structure to the data: identifies how data is grouped and accessed.
Attributes are organized into:

* **Characteristic**: a single piece of data that a device can share.
* **Service**: a collection of related characteristics
* **Profiles**: a collection of related services

Example:

> Heart Rate Profile => Device Info Service (make, model, sn) ; Heart Rate Service (heart rate measurement; body sensor location)

Each service and characteristic should have a unique ID value.
The UUID could be either a *standard Bluetooth-SIG defined UUID* (16-bit)
or a custom UUID (128 bit).

See: [Assigned Numbers](https://www.bluetooth.com/specifications/assigned-numbers/) — the list of pre-defined UUIDs.

### BLE Address

BLE Address is like a MAC address: 48 bit hex.
Two main types of BLE addresses:

* Public Address: A public address is a permanent, worldwide-unique code given to a device by its manufacturer.
  It never changes and is registered with the IEEE. Only one device can have each address, and getting one requires paying a fee.
* Random Address: A random address is used more often because you don't need to register it with the IEEE, and you can set it up yourself.

  Random addresses can be further classified into:

  * Static: Stays the same until you restart the device
  * Private (dynamic): Changes over time to protect your privacy.
    Random addresses help to protect privacy by hiding the device's real identity.

# Advertisement

A BLE device advertises itself.
There are 3 binary options:

* Connectable/Unconnectable: Client can connect and establish a GATT session (read/write characteristics)?
  Unconnectable: Broadcast only, no connections allowed (beacon mode)
* Scannable/Nonscannable: Client can request more info with a scan request (gets scan response packet).
  Nonscannable: Advertising data only, no additional scan response
* Directed/Undirected: Advertisement targets a specific device (by address) - fast reconnection.
  Undirected: Broadcast to everyone.

Examples:

* Connectable + Scannable + Undirected:
  Normal peripheral device. Advertises to everyone, allows connections, provides extra info on scan.
* Non-connectable + Non-scannable:
   Beacon mode. Just broadcasts data (temperature sensor, iBeacon). No connections.
* Connectable + Directed:
  Fast reconnect to known device. "Hey, device X, I'm here, connect immediately."

Why it matters:

* Beacons: non-connectable saves power (no connection overhead)
* Scannable: extra 31 bytes for device name/info without connecting
* Directed: millisecond reconnects vs seconds for undirected





## Rust Crates

* [Bleps](https://github.com/bjoernQ/bleps): A toy-level BLE peripheral stack. Easy to use. No async. Only ESP32.
* [Trouble](https://github.com/embassy-rs/trouble): Hardware-agnostic. More feature-complete. Async. Portable across chips.

## Links

* [The Bluetooth® Low Energy Primer](https://www.bluetooth.com/bluetooth-resources/the-bluetooth-low-energy-primer/)
* [BLE Fundamentals](https://academy.nordicsemi.com/courses/bluetooth-low-energy-fundamentals/)
* [More Examples: embassy-rs/trouble/examples](https://github.com/embassy-rs/trouble/tree/main/examples/esp32)

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





# topics/components
# PIR Sensor (Motion Sensor)

PIR = Passive Infrared: because it does not emit any IR radiation; instead, it
only detects changes in infrared radiation from the environment. It senses heat emitted by people, animals, and other warm objects.

It has 3 pins:

1. Power: 5V
2. Output: HIGH when motion is detected, LOW otherwise. Conveniently, it's 3.3V
3. Ground

It has 2 potentiometeres:

1. Delay Time (Output Duration): determines how long the sensor’s output remains HIGH after detecting motion.
   A longer delay is useful for applications like automatic lighting, where you want the light to stay on for a while after movement is detected. Range: 5s .. 200s.
2. Sensitivity (Detection Range): controls how far the sensor can detect motion. Higher values => more distance, but also more false triggers. Range: 3m .. 7m.

Jumper setting:

1. Retriggering: the output stays HIGH as long as motion is detected. Use case: keep the lights on.
2. Non-Retriggering: the output stays HIGH once motion is detected but won't trigger again until the delay time is up. Use case: counting people.


# SD Card

## Bus Mode
SD Cards can work in SPI mode!

All SD card families initially use a 3.3 volt electrical interface.
On command, SDHC and SDXC cards can switch to 1.8 V operation.

Like most memory card formats, SD is covered by patents and trademarks. Royalties apply to the manufacture
and sale of SD cards and host adapters, with the exception of SDIO devices.
NOTE: SPI mode does not require a host license. SDcard mode does. Also check: MMC mode.

The full details of the SD Bus protocol are not publicly available and can only be accessed through the SD Association.

## Command Interface

SD cards and host devices initially communicate through a synchronous one-bit interface, where the host device
provides a clock signal that strobes single bits in and out of the SD card. The host device thereby sends 48-bit
commands and receives responses. The card can signal that a response will be delayed, but the host device can abort the dialogue.

Through issuing various commands, the host device can:
* Determine the type, memory capacity and capabilities of the SD card
* Command the card to use a different voltage, different clock speed, or advanced electrical interface
* Prepare the card to receive a block to write to the flash memory, or read and reply with the contents of a specified block.

The command interface is an extension of the MultiMediaCard (MMC) interface. SD cards dropped support for some of the commands
in the MMC protocol, but added commands related to copy protection. By using only commands supported by both standards
until determining the type of card inserted, a host device can accommodate both SD and MMC cards.

At power-up or card insertion, the voltage on pin 1 selects either the Serial Peripheral Interface (SPI) bus or the SD bus.
The SD bus starts in one-bit mode, but the host device may issue a command to switch to the four-bit mode, if the SD card supports it.

## Clock Speed
After determining that the SD card supports it, the host device can also command the SD card to switch
to a higher transfer speed. Until determining the card's capabilities, the host device should not use
a clock speed faster than 400 kHz.

SD cards other than SDIO (see below) have a "Default Speed" clock rate of 25 MHz.

## Power Consumption

The power consumption of SD cards varies by its speed mode.

During transfer it may be in the range of 66–330 mW (20–100 mA at a supply voltage of 3.3 V).
Standby current is much lower, less than 0.2 mA for one 2006 microSD card.

Modern UHS-II cards can consume up to 2.88 W, if the host device supports bus speed mode SDR104 or UHS-II.
Minimum power consumption in the case of a UHS-II host is 720 mW.

## Security
The host device can command the SD card to become read-only: reversible or permanent.
Most full-size SD cards have a mechanical write-protect switch.

A host device can lock an SD card using a password of up to 16 bytes.
A locked card rejects commands to read and write data.

## Pins

1. -
2. CS: Chip Select
3. DI: MOSI
4. VDD: +3.3V
5. SCK
6. GND
7. DO: MISO
8. -


# SD Card Markings

## Capacity standards

* SDSC: max 2 Gb, FAT12, FAT16
* SDHC: max 32 Gb, FAT32
* SDXC: max 2 Tb, exFAT (required by the SDXC standard, but can be reformatted)
* SDUC: max 128 Tb, exFAT

SDHC cards are physically identical to SD (SDSC) cards.

### Filesystem

Note on filesystem: reformatting an SD card may make the card slower, or shorten its lifespan!
Some cards use *wear-leveling algorithms* that are designed for the access patterns typical of FAT12, FAT16 or FAT32.
In addition, the preformatted file system may use a cluster size that matches the erase region of the physical memory on the card;
reformatting may change the cluster size and make writes less efficient.

The SD Association provides freely downloadable SD Formatter software to overcome these problems for Windows and Mac OS X.

## Bus Marks
Most relevant for handling large files.

* Default: 12.5 Mb/s — the original SD bus interface
* High-Speed: 25 Mb/s
* UHS-I, UHS-II, UHS-III: 50 Mb/s, 150 Mb/s, 312 Mb/s
* SD Express: 985 MB/s

UHS cards and devices use specialized electrical signaling and hardware interfaces.

* UHS-I cards operate at 1.8 V instead of the standard 3.3 V and use a four-bit transfer mode.
* UHS-II and UHS-III introduce a second row of interface pins to add a second lane of data transfer
  and use low-voltage differential signaling (LVDS) at 0.4 V to increase speed and reduce power consumption and electromagnetic interference (EMI).

SD Express incorporates a single PCI Express 3.0 (PCIe) lane and supports the NVM Express (NVMe) storage protocol.
They also support DMA (direct memory access).

## Speed Ratings

Speed classes overlap:

* C: Original speed class. C2, C4, C6, C10. Speeds: 2-10 Mb/s (minimum sustained write speed)
* U: UHS speed class. U1 = 10 Mb/s, U3 = 30 Mb/s
* V: Video speed class. V6, V10, V30, V60, and V90 = 6 Mb/s, ..., 90 Mb/s
* E: SD Express Speed Class. E150, E300, E450, and E600 — minimum sustained write speed

# Servo

The SG90 offers up to 180° of rotation.

A servo motor usually has a DC motor, a control circuit, a gearbox, and a potentiometer.
The DC motor is linked to the output shaft through the gearbox, which moves the servo's horn.

To control the horn's position, we send a signal to the servo motor from the microcontroller (MCU)
at a frequency of 50Hz, with a pulse every 20 milliseconds. By changing how long the signal stays high
during each cycle (pulse width), we can control how far the horn rotates.

The servo motor holds its position until we change the pulse width.
If you send the wrong pulse width, it won't move at all.

Pins:

1. Orange = PWM
2. Red = VCC
3. Brown = GND

In the datasheet for SG90, the pulse pattern is:

* 1ms pulse => -90°
* 1.5ms pulse => 0°
* 2ms pulse => +90°

My actual motor, however, reacts differently:

* 0.625ms => -90°
* 2.575ms => +90°

At 50 Hz and max_duty=4096 (12 bits), this translates to:

* duty=128/4096 => -90°
* duty=527/4096 => +90°

# OLED Display

OLED: Organic Light-Emitting Diode.

OLED displays consume less power and offer better performance than LCDs
since they don't require a backlight.

## I2C

For I2C, two GPIO pins need to be configured as SDA (Serial Data) and SCL (Serial Clock).
On ESP32-C3, only GPIO 8 and 9 can be used for I2C.
So:

* GPIO8 → SDA
* GPIO9 → SCL
* 3.3V → VCC
* GND → GND

It doesn't require any pull-up resistors because they're included on the board itself.

## GDDRAM

In the datasheet, the 128 columns are referred to as segments,
while the 64 rows are called commons.

The OLED display's pixels are arranged in a page structure within GDDRAM (Graphics Display DRAM).
GDDRAM is divided into 8 pages (From Page 0 to Page 7), each consisting of 128 columns (segments) and 8 rows (commons).

A segment is 8 bits of data (one byte), with each bit representing a single pixel.
When writing data, you will write an entire segment, meaning the entire byte is written at once.

We can, of course, re-map both segments and commons through software.

Use crates:

* [`ssd1306`](https://docs.rs/ssd1306/latest/ssd1306/): a driver interface for the SSD1306 monochrome OLED display,
  supporting both I2C and SPI through the [`display_interface`](https://docs.rs/display-interface/latest/display_interface/) crate.
  It also has async support that you have to enable through the feature flag.
* [`embedded_graphics`](https://docs.rs/embedded-graphics/latest/embedded_graphics/):
  a 2D graphics library that is focused on memory constrained embedded devices.
  Its goal is to draw graphics without using any buffers and pre-allocated memory.
  This is achieved with an iterator-based approach.w

```rust
// I2C. Init in async mode.
let i2c_bus = i2c::master::I2c::new(
    peripherals.I2C0,
    i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
).expect("Init I2C bus").with_sda(peripherals.GPIO8).with_scl(peripherals.GPIO9).into_async();

// Init SSD1306 driver. Pass i2c bus to it.
// Use a "buffered graphics" mode, supporting embedded-graphics.
let display_iface = ssd1306::I2CDisplayInterface::new(i2c_bus);
let mut display = ssd1306::Ssd1306Async::new(
    display_iface,
    ssd1306::size::DisplaySize128x64,
    ssd1306::rotation::DisplayRotation::Rotate0,
).into_buffered_graphics_mode();
display.init_with_addr_mode(ssd1306::command::AddrMode::Page).await.expect("Display init failed");
```

LCD (I2C)
=========

Device: Hitachi HD44780 compatible LCD

* Can display ASCII + up to 8 custom characters
* Variants: 16x2, 20x4, different backlight colors
* Some of them come with an integrated I2C interface adapter

Parallel Interface
-------------------

At its core, the HD44780 controller uses a parallel interface.
This is the native and most direct way to communicate with the LCD.
Many modules expose this interface directly through their 16-pin header.

How It Works
------------

Each character is a 5x8 pixel grid.
You don't have to control pixels: the controller maps ASCII characters to pixel grids.
It can store 8 custom characters as well.

It supports two data modes:

* 8-bit mode: data is sent as a full byte using all the data pins. Uses more wires but is faster.
* 4-bit mode: only the higher-order data bits are used, sending data in nibbles.

I2C Module
----------

Pins:

* VCC, GND: Power supply (5V)
* SDA, SCL: I2C data and clock lines

Potentiometer: use to adjust contrast if the text is not clear.






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





# tutorials/b03-buzzer/src/bin


# tutorials/b03-buzzer/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use b03_buzzer::{music, nokia};
use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();

use log::info;
use esp_hal::{
    time::{Duration, Instant, Rate},
    clock::CpuClock,
    gpio,
    ledc,
    main,
};

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Buzzer pin
    let mut buzzer_pin = peripherals.GPIO8;

    // Active buzzer: just give it voltate.
    // NOTE: comment it out because it consumes the pin
    let mut buzzer = gpio::Output::new(buzzer_pin, gpio::Level::High,
        // Use OpenDrain if GPIO is connected to the transistor's base and there's external pull up.
        // Oherwise it will give a continuous buzz and that's it.
        gpio::OutputConfig::default()
        .with_drive_mode(gpio::DriveMode::OpenDrain)
    );
    for i in 1..30 {
        buzzer.toggle();
        busy_wait(Duration::from_millis(i*5));
    }


    // Init LEDC
    let mut ledc = ledc::Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk); // nothing works without this line!

    // Let's play a melody
    let song = music::Song::new(nokia::TEMPO);
    for (note, duration_type) in nokia::MELODY {
        // Get music note
        let note_duration = song.calc_note_duration(duration_type) as u64;
        let pause_duration = note_duration / 10; // 10% of note_duration
        if note == music::REST {
            busy_wait(Duration::from_millis(note_duration));
            continue;
        }
        let freq = Rate::from_hz(note as u32);

        // Prepare PWM
        // Note that we can't just keep re-using `buzzer` because it's consumed by the function.
        // We do reborrow()
        let buzzer = buzzer_pin.reborrow();
        let mut pwm_timer = ledc.timer::<ledc::LowSpeed>(ledc::timer::Number::Timer0);
        let mut pwm_channel = ledc.channel(ledc::channel::Number::Channel0, buzzer);
        use ledc::timer::TimerIFace;  // brings: .configure()
        use ledc::channel::ChannelIFace;  // brings: .configure()

        // Configure timer, channel.
        pwm_timer.configure(ledc::timer::config::Config {
            clock_source: ledc::timer::LSClockSource::APBClk,
            duty: ledc::timer::config::Duty::Duty10Bit,
            frequency: freq, // play the frequency
        }).unwrap();
        pwm_channel.configure(ledc::channel::config::Config {
            timer: &pwm_timer, // use the timer
            duty_pct: 50,
            drive_mode: gpio::DriveMode::PushPull,
        }).unwrap();

        // Play
        busy_wait(Duration::from_millis(note_duration - pause_duration)); // play 90%

        // Pause.
        // Disable PWM by setting duty=0: effectively, no signal
        pwm_channel.set_duty(0).unwrap();
        busy_wait(Duration::from_millis(pause_duration)); // Pause for 10%
    }

    loop{
        busy_wait(Duration::from_millis(100));
    }
}

// Blocking delay: burns CPU cycles until `duration` passes.
fn busy_wait(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}

```





# tutorials/b03-buzzer/src


# tutorials/b03-buzzer/src/nokia.rs

```rust
use crate::music::*;

pub const TEMPO: u16 = 180;

// Nokia Ringtone
pub const MELODY: [(f64, i16); 13] = [
    (NOTE_E5, 8),
    (NOTE_D5, 8),
    (NOTE_FS4, 4),
    (NOTE_GS4, 4),
    (NOTE_CS5, 8),
    (NOTE_B4, 8),
    (NOTE_D4, 4),
    (NOTE_E4, 4),
    (NOTE_B4, 8),
    (NOTE_A4, 8),
    (NOTE_CS4, 4),
    (NOTE_E4, 4),
    (NOTE_A4, 2),
];
```





# tutorials/b04-ultrasonic-distance/src/bin


# tutorials/b04-ultrasonic-distance/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();

use log;
use esp_hal::{
    delay::Delay,
    rtc_cntl::Rtc,
    clock::CpuClock, gpio, ledc,
    time::{Duration, Rate},
    main,
};

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // PWM for the buzzer
    let mut buzzer_pin = peripherals.GPIO4;
    let mut ledc = ledc::Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk); // nothing works without this line!

    // Ultrasonic sensor:
    // - TRIG pin: send a short pulse (10us) to trigger a reading
    // - ECHO pin: will respond with a continuous pulse. It's length is proportional to the distance.
    //   The initial state of this pin will be set to Pull Down to ensure it starts in the low state.
    let ultrasonic_trig_pin = peripherals.GPIO0;
    let mut ultrasonic_trig = gpio::Output::new(ultrasonic_trig_pin, gpio::Level::Low, gpio::OutputConfig::default());
    let ultrasonic_echo_pin = peripherals.GPIO1;
    let ultrasonic_echo = gpio::Input::new(ultrasonic_echo_pin, gpio::InputConfig::default()
        .with_pull(gpio::Pull::Down));

    // RTC time and delay
    let delay = Delay::new();
    let rtc = Rtc::new(peripherals.LPWR);

    // Keep measuring
    loop {
        // Send a TRIG to the ultrasonic module.
        // The datasheet requires it to be at least 10us long.
        ultrasonic_trig.set_low();
        delay.delay_micros(2);
        ultrasonic_trig.set_high();
        delay.delay_micros(10);
        ultrasonic_trig.set_low();

        // Now measure the response pulse width:
        // 1. Wait while it's low. Then record the time.
        // 2. Wait while it's high. Then record the time.
        while ultrasonic_echo.is_low() {}
        let time1 = rtc.current_time_us();
        while ultrasonic_echo.is_high() {}
        let time2 = rtc.current_time_us();
        let pulse_width = Duration::from_micros(time2 - time1);

        // The pulse width tells us how long it took for the ultrasonic waves to travel
        // to an obstacle and return. Since the pulse represents the round-trip time,
        // we divide it by 2 to account for the journey to the obstacle and back.
        let distance_cm = (pulse_width.as_micros() as f64 * 0.0343) / 2.0;  // speed of sound / 2

        // Convert distance to frequency
        const MIN_DISTANCE: f64 = 10.0;
        const MAX_DISTANCE: f64 = 200.0;
        const MIN_FREQ: u32 = 700;
        const MAX_FREQ: u32 = 7000;
        let freq = Rate::from_hz(match distance_cm {
            n if n < MIN_DISTANCE => MIN_FREQ,
            n if n > MAX_DISTANCE => MAX_FREQ,
            n => MIN_FREQ + (MAX_FREQ as f64 * n/MAX_DISTANCE) as u32,
        });
        let duty = match distance_cm {
            n if n > MAX_DISTANCE => 0,
            _ => 10,
        };
        log::info!("distance={distance_cm}cm freq={freq} duty={duty}");

        // Configure PWM to give us SOUND!
        let buzzer = buzzer_pin.reborrow();
        let mut pwm_timer = ledc.timer::<ledc::LowSpeed>(ledc::timer::Number::Timer0);
        let mut pwm_channel = ledc.channel(ledc::channel::Number::Channel0, buzzer);
        use ledc::timer::TimerIFace;  // brings: .configure()
        use ledc::channel::ChannelIFace;  // brings: .configure()
        pwm_timer.configure(ledc::timer::config::Config {
            clock_source: ledc::timer::LSClockSource::APBClk,
            duty: ledc::timer::config::Duty::Duty7Bit,
            frequency: freq,
        }).unwrap();
        pwm_channel.configure(ledc::channel::config::Config {
            timer: &pwm_timer,
            duty_pct: duty,
            drive_mode: gpio::DriveMode::PushPull,
        }).unwrap();

        // Sleep a bit. Don't measure too often.
        // This is also a requirement from the ultrasonic module: interval between trigger pulses.
        delay.delay_millis(60);
    }
}

```





# tutorials/b05-pir-sensor/src/bin


# tutorials/b05-pir-sensor/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
esp_bootloader_esp_idf::esp_app_desc!();
use {esp_backtrace as _, esp_println as _};

// This time we'll use defmt
use defmt;
use esp_hal::{
    delay,
    gpio,
    main,
};

#[main]
fn main() -> ! {
    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // PIR Sensor.
    // With an initial state of "pull down": it goes HIGH when motion is detected.
    let pir_sensor = gpio::Input::new(peripherals.GPIO0, gpio::InputConfig::default()
        .with_pull(gpio::Pull::Down));

    let delay = delay::Delay::new();
    loop {
        if pir_sensor.is_high() {
            defmt::info!("Motion detected");
            // TODO: You can trigger the buzzer here.
            delay.delay_millis(100);
        }
        delay.delay_millis(100);
    }
}

```





# tutorials/b06-servo/src/bin


# tutorials/b06-servo/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();

use log;
use esp_hal::{
    delay::Delay,
    clock::CpuClock, gpio, ledc,
    time::{Duration, Rate},
    rng::Rng,
    main,
};

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    let servo_pin = peripherals.GPIO0;

    // PWM
    let mut ledc = ledc::Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk); // nothing works without this line!
    let mut pwm_timer = ledc.timer::<ledc::LowSpeed>(ledc::timer::Number::Timer0);
    let mut pwm_channel = ledc.channel(ledc::channel::Number::Channel0, servo_pin);
    use ledc::timer::TimerIFace;  // brings: .configure()
    use ledc::channel::ChannelIFace;  // brings: .configure()

    // Configure timer, channel.
    pwm_timer.configure(ledc::timer::config::Config {
        clock_source: ledc::timer::LSClockSource::APBClk,
        duty: ledc::timer::config::Duty::Duty12Bit,
        frequency: Rate::from_hz(50),
    }).unwrap();
    pwm_channel.configure(ledc::channel::config::Config {
        timer: &pwm_timer,
        duty_pct: 0,
        drive_mode: gpio::DriveMode::PushPull,
    }).unwrap();

    // We need to control the pulse width using duty cycle.
    // Max duty cycle depends on the duty resolution bits we've configured.
    use embedded_hal::pwm::SetDutyCycle;
    let max_duty_cycle = pwm_channel.max_duty_cycle();

    // Convert our servo's extreme values into duty cycle range
    const SERVO_MIN: Duration = Duration::from_micros(600);
    const SERVO_MAX: Duration = Duration::from_micros(2575);
    const PWM_FREQ: Rate = Rate::from_hz(50);
    let min_duty: u16 = (max_duty_cycle as f64 * (SERVO_MIN.as_micros() as f64) / (PWM_FREQ.as_duration().as_micros() as f64)) as u16;
    let max_duty: u16 = (max_duty_cycle as f64 * (SERVO_MAX.as_micros() as f64) / (PWM_FREQ.as_duration().as_micros() as f64)) as u16;

    log::info!("max_duty_cycle={max_duty_cycle}");
    log::info!("min_duty={min_duty}");
    log::info!("max_duty={max_duty}");

    // Go
    let delay = Delay::new();
    let rng = Rng::new();
    loop {
        // Gen random angle
        let angle = (rng.random() % 180) as u8;

        // Go to angle
        let duty = duty_from_angle(angle, min_duty, max_duty);
        pwm_channel.set_duty_cycle(duty).unwrap();

        // Sleep
        delay.delay_millis(500);
    }
}


// Angle => Duty perc
// 0 .. 180
fn duty_from_angle(deg: u8, min_duty: u16, max_duty: u16) -> u16 {
    let k = deg as f32 / 180.0;  // 0..100
    min_duty + ((max_duty-min_duty) as f32 * k) as u16
}

```





# tutorials/b07-adc-ldr/src/bin


# tutorials/b07-adc-ldr/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]

use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();

use log;
use esp_hal::{
    time::{Instant, Duration},
    clock::CpuClock,
    gpio, analog::adc,
    delay::Delay,
    main,
};
use nb;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // LED
    let mut onboard_led = gpio::Output::new(peripherals.GPIO8, gpio::Level::Low, gpio::OutputConfig::default());

    // ADC Input
    // Attenuation: -11dB
    let ldr_pin = peripherals.GPIO0;
    let mut adc_config = adc::AdcConfig::new();
    let mut pin = adc_config.enable_pin(ldr_pin, adc::Attenuation::_11dB);
    let mut adc1 = adc::Adc::new(peripherals.ADC1, adc_config);

    let delay = Delay::new();
    loop {
        // Read
        // NOTE: ADC would sometimes return Err(WouldBlock). This we ignore.
        match adc1.read_oneshot(&mut pin) {
            Ok(result) => log::info!("adc={result}"),
            Err(nb::Error::WouldBlock) => continue,
            Err(err) => log::error!("err={err:?}"),
        }

        // Instead, we can use nb::block!() to wait until the value becomes available.
        // Internally, it loop{}s until WouldBlock goes away and a value becomes available.
        let luminosity: u16 = match nb::block!(adc1.read_oneshot(&mut pin)) {
            Ok(result) => {
                (result.max(1900)-1900)/20
            }
            Err(err) => {
                log::error!("err={err:?}");
                continue;
            }
        };
        log::info!("luminosity={luminosity}");

        // Blink LED
        let now = Instant::now();
        while now.elapsed() < Duration::from_millis(300) {
            onboard_led.toggle();
            delay.delay_millis(luminosity as u32);
        }
    }
}

```





# tutorials/b08-wifi-http-client/src/bin


# tutorials/b08-wifi-http-client/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![feature(impl_trait_in_assoc_type)]
use {esp_backtrace as _, esp_println as _};
esp_bootloader_esp_idf::esp_app_desc!();

// Memory allocations
// Use this create instead for heap allocations: Box, Rc, RefCell, Arc.
// esp-radio needs it.
extern crate alloc;

use defmt;
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    rng::Rng,
    interrupt::software::SoftwareInterruptControl,
};

// Embassy
// $ cargo add esp-hal --features unstable   # requires unstable features
// $ cargo add esp-rtos --features esp32c3,embassy,log-04
// $ cargo add embassy-executor --features nightly
// $ cargo add embassy-time
// $ cargo add embassy-net --features defmt,tcp,udp,dhcpv4,dhcpv4-hostname,medium-ethernet,dns
// $ cargo add smoltcp --features dns-max-server-count-4
// $ cargo add reqwless --features embedded-tls,defmt
use esp_rtos;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_net::{dns::DnsSocket, tcp::client::{TcpClient, TcpClientState}};

// Reqwless: HTTP client
use reqwless::client::{HttpClient, TlsConfig};

// Our library
use b08_wifi_http_client as lib;


#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // Init allocator: 64K in reclaimed memory + 66K in default RAM
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 64 * 1024);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    // CPU Clock: WiFi in ESP32 requires a fast CPU
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // Init Embassy the usual way
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    // Init WiFi & network stack
    let stack = lib::wifi::start_wifi(&spawner, peripherals.WIFI).await;
    let rng = Rng::new();
    let tls_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    // Load the website
    load_url(stack, tls_seed).await;

    // Go
    loop {
        Timer::after(Duration::from_millis(500)).await;
    }
}

// HTTP request using the network stack and reqwless.
// HTTP Request.
// First, initialize the DNS socket and the TCP client.
// Now we init TLS configuration using a random number.
async fn load_url(stack: embassy_net::Stack<'_>, tls_seed: u64) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let dns = DnsSocket::new(stack);
    let tcp_state = TcpClientState::<1, 4096, 4096>::new();
    let tcp = TcpClient::new(stack, &tcp_state);

    let tls = TlsConfig::new(
        tls_seed,
        &mut rx_buffer,
        &mut tx_buffer,
        reqwless::client::TlsVerify::None,
    );

    let mut client = HttpClient::new_with_tls(&tcp, &dns, tls);
    let mut buffer = [0u8; 4096];
    let mut http_req = client
        .request(
            reqwless::request::Method::GET,
            "https://jsonplaceholder.typicode.com/posts/1",
        )
        .await
        .unwrap();
    let response = http_req.send(&mut buffer).await.unwrap();

    defmt::info!("Got response");
    let res = response.body().read_to_end().await.unwrap();

    let content = core::str::from_utf8(res).unwrap();
    defmt::println!("{}", content);
}

```





# tutorials/b08-wifi-http-client/src


# tutorials/b08-wifi-http-client/src/lib.rs

```rust
#![no_std]
#![feature(impl_trait_in_assoc_type)]

pub mod wifi;

// Use mk_static!() macro to do esp_radio::init() with a static lifetime.
// The StaticCell crate is useful when you need to initialize a variable at runtime
// but require it to have a static lifetime.
// We will define a macro to create globally accessible static variables.
// Args:
// - Type of variable
// - The value to initialize it with
// The uninit function provides a mutable reference to the uninitialized memory, and we write the value into it.
//
// NOTE: the nightly version of the package contains mk_static!()
#[macro_export]
macro_rules! make_static {
    ($t:ty, $val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

```



# tutorials/b08-wifi-http-client/src/wifi.rs

```rust
use defmt;
use esp_hal::{
    rng::Rng,
};
use crate::make_static;

use embassy_executor::Spawner;
use esp_radio::wifi;
use embassy_time::{Duration, Timer};
use core::{net::Ipv4Addr, str::FromStr};
use embassy_net::{DhcpConfig, Ipv4Cidr, Stack, StaticConfigV4};


// anyhow: return errors
use anyhow::{Context, Result, anyhow};

// Run me:
// $ SSID="name" PASSWORD="secret" cargo run
// $ SSID="name" PASSWORD="secret" STATIC_IP=192.168.0.199/24 GATEWAY=192.168.0.1 cargo run
// $ AP_MODE=1 SSID="esp32" PASSWORD="12345678" STATIC_IP=192.168.12.1/24 GATEWAY=192.168.12.1 cargo run


// Load WiFi credential from environment variables
// Variables will be read *at compile time*
const AP_MODE: Option<&str> = option_env!("AP_MODE");  // run in AP mode
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");
const STATIC_IP: Option<&str> = option_env!("STATIC_IP"); // optional
const GATEWAY_IP: Option<&str> = option_env!("GATEWAY"); // optional

// The number of sockets to allocate enough space for.
const N_SOCKETS: usize = 7;

// Start WiFi, spawn net tasks, return net stack
pub async fn start_wifi(
    spawner: &Spawner,
    wifi_peripheral: esp_hal::peripherals::WIFI<'static>,
) -> Result<Stack<'static>> {
    // Init Wifi.
    // Make a static variable: it's globally available.
    let radio_init = &*make_static!(
        esp_radio::Controller<'static>,
        esp_radio::init()
            // Use .context() to wrap the error
            .context("Failed to initialize the radio")?
            // .map_err(|e| anyhow!("Failed to initialize the radio: {}", e))?
    );

    // Init controller
    let (wifi_controller, interfaces) =
        wifi::new(&radio_init, wifi_peripheral, Default::default())
            .context("Failed to initialize Wi-Fi controller")?;
    let wifi_interface = if AP_MODE.is_none() {
        interfaces.sta // WiFi station: the client
    } else {
        interfaces.ap
    };

    // Network stack needs a random number: for TLS and networking.
    // The net stack wants a u64, so we join two u32-s.
    let rng = Rng::new();
    let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    // Init network stack
    let net_config = {
        // Static ip
        if let Some(ip) = STATIC_IP && let Some(gw) = GATEWAY_IP {
            embassy_net::Config::ipv4_static(StaticConfigV4{
                address: Ipv4Cidr::from_str(ip).unwrap(),
                gateway: Ipv4Addr::from_str(gw).ok(),
                dns_servers: Default::default(), // TODO: use Google DNS by default?
            })
        // DHCP
        } else {
            embassy_net::Config::dhcpv4({
                let mut c = DhcpConfig::default();
                c.hostname = Some(heapless::String::new());
                c
            })
        }
    };
    let (stack, runner) = embassy_net::new(
        wifi_interface, net_config,
        // Stack resources: size=3
        make_static!(embassy_net::StackResources<N_SOCKETS>, embassy_net::StackResources::<N_SOCKETS>::new()),
        net_seed,
    );

    // Start two background tasks:
    // - the connection_task will maintain the Wi-Fi connection
    // - the net_task will run the network stack and handle network events.
    if AP_MODE.is_none() {
        spawner.spawn(task_keep_wifi_client_up(wifi_controller)).ok();
    } else {
        spawner.spawn(task_keep_wifi_ap_up(wifi_controller)).ok();
    }
    spawner.spawn(task_network(runner)).ok();

    // Wait until the connection is up
    wait_for_connection(stack).await;

    // Done
    Ok(stack)
}


// Task: run the network stack
#[embassy_executor::task]
async fn task_network(mut runner: embassy_net::Runner<'static, wifi::WifiDevice<'static>>) {
    runner.run().await
}


// Task: manage WiFi connection by continuously checking the status, configuring the Wi-Fi controller,
// and attempting to reconnect if the connection is lost or not started.
#[embassy_executor::task]
async fn task_keep_wifi_client_up(mut controller: wifi::WifiController<'static>) {
    defmt::info!("WiFi: start client");
    defmt::info!("WiFi: Device capabilities: {:?}", controller.capabilities());

    loop {
        // 1. Check WiFi state
        // If it is in StaConnected, we wait until it gets disconnected.
        match wifi::sta_state() {
            wifi::WifiStaState::Connected => {
                // wait until we're no longer connected
                controller.wait_for_event(wifi::WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await;
            }
            _ => {},
        }

        // 2. Check if the WiFi controller is started.
        // If not, we initialize the WiFi client configuration.
        if !matches!(controller.is_started(), Ok(true)) {
            // Init client. Use SSID.
            let client_config = wifi::ModeConfig::Client(
                wifi::ClientConfig::default()
                    .with_ssid(SSID.into())
                    .with_password(PASSWORD.into())
                    .with_auth_method(wifi::AuthMethod::Wpa2Personal),  // TODO: configurable?
            );
            controller.set_config(&client_config).unwrap();
            defmt::debug!("WiFi: starting...");

            // Wifi start.
            controller.start_async().await.unwrap();
        }

        // Wait until connected
        defmt::debug!("WiFi: connecting...");
        match controller.connect_async().await {
            // NOTE: This is only WiFi.
            // The network stack (smoltcp) will need to use its DHCP client now.
            Ok(_) => {
                let rssi = controller.rssi().unwrap_or(-999);
                defmt::info!("WiFi: connected! rssi={}", rssi);
            }
            Err(e) => {
                defmt::warn!("WiFi: failed to connect: {:?}", e);

                // Sleep 5s before trying again
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

// Task: wait for the Wi-Fi link to be up, then obtain the IP address.
async fn wait_for_connection(stack: embassy_net::Stack<'_>) {
    let mut print_once: bool = false;
    while !stack.is_link_up() {
        if !print_once {
            defmt::debug!("Net: waiting on link ...");
            print_once = true;
        }
        Timer::after(Duration::from_millis(100)).await;
    }

    let mut print_once: bool = false;
    // while !stack.is_config_up() {
    //     Timer::after(Duration::from_millis(100)).await
    // }
    loop {
        if let Some(config) = stack.config_v4() {
            defmt::info!("IP: {}", config.address);
            break;
        }
        if !print_once {
            defmt::debug!("Net: waiting for network config ...");
            print_once = true;
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}





#[embassy_executor::task]
async fn task_keep_wifi_ap_up(mut controller: wifi::WifiController<'static>) {
    defmt::info!("WiFi: start AP");

    loop {
        // WiFi AP is up and running? Then do nothing.
        if wifi::ap_state() == wifi::WifiApState::Started {
            Timer::after(Duration::from_millis(1000)).await;
            continue;
        }

        // Re-configure AP
        if !matches!(controller.is_started(), Ok(true)) {
            // Init client. Use SSID.
            let ap_config = wifi::ModeConfig::AccessPoint(
                wifi::AccessPointConfig::default()
                    .with_ssid(SSID.into())
                    .with_password(PASSWORD.into())
                    .with_auth_method(wifi::AuthMethod::Wpa2Wpa3Personal),
            );
            controller.set_config(&ap_config).unwrap();
            defmt::info!("WiFi: Starting ...");

            // Wifi start.
            controller.start_async().await.unwrap();
        }
    }
}

```





# tutorials/b09-wifi-http-server/src/bin


# tutorials/b09-wifi-http-server/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![feature(impl_trait_in_assoc_type)]
esp_bootloader_esp_idf::esp_app_desc!();
use {esp_backtrace as _, esp_println as _};
extern crate alloc;

use defmt;
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    interrupt::software::SoftwareInterruptControl,
    gpio,
};

// $ cargo add embassy-executor --features task-arena-size-65536
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

// Our library
use b08_wifi_http_client::wifi;
use b09_wifi_http_server::{webserver, led};

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 64 * 1024);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // RTOS
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    // Init WiFi & network stack
    let stack = wifi::start_wifi(&spawner, peripherals.WIFI).await;

    // Spawn webserver tasks
    let web_app = webserver::WebApp::default();
    for id in 0..webserver::WEB_TASK_POOL_SIZE {
        spawner.must_spawn(webserver::web_task(id, stack, web_app.router, web_app.config));
    }
    defmt::info!("Web server started!");

    // Spawm LED tasks
    spawner.must_spawn(led::led_task(
        gpio::Output::new(peripherals.GPIO8, gpio::Level::Low, gpio::OutputConfig::default()),
    ));

    // Sleep
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

```





# tutorials/b09-wifi-http-server/src


# tutorials/b09-wifi-http-server/src/led.rs

```rust
use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;

use core::sync::atomic::{AtomicBool, Ordering};

// Primitive shared-memory communication:
// 1. webserver will set the value
// 2. LED task will read it
pub static LED_STATE: AtomicBool = AtomicBool::new(false);

// Task: toggle LED, controlled by static `LED_STATE`
#[embassy_executor::task]
pub async fn led_task(mut led: Output<'static>) {
    // NOTE: reading a GPIO state is as easy as reading a mem region.
    // Therefore we don't have to keep this bool here: just read current state and compare.
    let mut current_led_state: bool = led.is_set_low(); // the LED is inverted
    loop {
        // Read expected LED state
        let set_led_state = LED_STATE.load(Ordering::Relaxed);

        // Set LED. Only if changed.
        match (current_led_state, set_led_state) {
            (true, false) => led.set_high(),
            (false, true) => led.set_low(),
            _ => ()
        }
        current_led_state = set_led_state;

        // Sleep
        Timer::after(Duration::from_millis(50)).await;
    }
}

```



# tutorials/b09-wifi-http-server/src/webserver.rs

```rust
use core::sync::atomic::Ordering;

use embassy_net::Stack;
use embassy_time::Duration;

// Picoserve: async http server for bare-metal environments.
// $ cargo add picoserve --features embassy
use picoserve::{
    AppBuilder, AppRouter, response::{File, IntoResponse}, routing::{self, Router}
};

// Serde: Serialize/Deserialize data structures
// $ cargo add serde --no-default-features --features derive
use serde;

// How many embassy tasks to spawm as http server workers?
pub const WEB_TASK_POOL_SIZE: usize = 2;


pub struct Application;

// Implement the `AppBuilder` trait for application: this creates a router.
// If you need a router with state, impl `AppWithStateBuilder`
impl AppBuilder for Application {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> Router<Self::PathRouter> {
        // Serve a static file.
        // Its contents are embedded.
        Router::new()
            .route(
                "/",
                routing::get_service(File::html(include_str!("index.html"))),
            )
            .route(
                "/set-led",
                routing::post(api_set_led),
            )
    }
}


// Request JSON
#[derive(serde::Deserialize)]
struct LedRequest {
    is_on: bool,
}

// Response JSON
#[derive(serde::Serialize)]
struct LedResponse {
    success: bool,
}

// API: set led
async fn api_set_led(input: picoserve::extract::Json<LedRequest, 0>) -> impl IntoResponse {
    // JSON value: put into LED_SATE
    crate::led::LED_STATE.store(input.0.is_on, Ordering::Relaxed);

    // Respond
    picoserve::response::Json(LedResponse { success: true })
}



// Web app: holds config and an instance of picoserve router
pub struct WebApp {
    pub router: &'static Router<<Application as AppBuilder>::PathRouter>,
    pub config: &'static picoserve::Config<Duration>,
}

impl Default for WebApp {
    fn default() -> Self {
        let router = picoserve::make_static!(AppRouter<Application>, Application.build_app());
        let config = picoserve::make_static!(
            picoserve::Config<Duration>,
            picoserve::Config::new(picoserve::Timeouts {
                start_read_request: Some(Duration::from_secs(5)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),
                persistent_start_read_request: Some(Duration::from_secs(1)),
            })
            .keep_connection_alive()
        );

        Self { router, config }
    }
}


// A pool of http server workers
#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
pub async fn web_task(
    id: usize,
    stack: Stack<'static>,
    router: &'static AppRouter<Application>,
    config: &'static picoserve::Config<Duration>,
) -> ! {
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve(
        id,
        router,
        config,
        stack, 80,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await
}

```





# tutorials/b10-i2c-oled-display
OLED Dino with an Accelerometer
===============================

This project was supposed to be a simple I2C OLED display test, but in the end it's the Chrome Dino game :D

1. Init I2C bus as a *shared bus*: i.e. multiple drivers can use it
2. Init the OLED display using a driver
3. Print out some text. Demo.
4. Starts a Dino game with sprites and animation
5. Spawns a thread that detects you jumping with the accelerometer

Wiring for ESP32-C3:

1. OLED display:
   * GND -> GND
   * VCC -> 3.3V
   * SCL -> GPIO9
   * SDA -> GPIO8

2. ADXL345 accelerometer:
   * GND -> GND
   * VCC -> 3.3V
   * CS -> self VCC
   * SDO -> self GND
   * SDA -> GPIO8
   * SCL -> GPIO9





# tutorials/b10-i2c-oled-display/src/bin


# tutorials/b10-i2c-oled-display/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
esp_bootloader_esp_idf::esp_app_desc!();
use {esp_backtrace as _, esp_println as _};

use defmt::info;
use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    i2c::{self, master::I2c},
    time::Rate,
    Async,
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Instant};

// SSD1306 display driver
use ssd1306;

// Embedded graphics: 2D graphics library.
// Can draw or print text on anything that implemements the `DrawTarget` trait.
use embedded_graphics::{
    Drawable,
    image::{Image, ImageRaw},
    mono_font,
    pixelcolor,
    prelude::{Point, Size},
    text::{Baseline, Text}
};
// TinyBMP: no-std, low-memory BMP loader
// $ cargo add tinybmp
use tinybmp;

// Use it to share the I2C bus
// $ cargo add embassy-sync --features defmt
// $ cargo add embassy-embedded-hal --features defmt
use embassy_sync::{
    mutex::Mutex,
    blocking_mutex::raw::NoopRawMutex,
};
use embassy_embedded_hal::{
    shared_bus::asynch::i2c::I2cDevice,
};
use static_cell::{ StaticCell };

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // RTOS
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt = esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);



    // === Init Shared I2C Bus === //

    // I2C. Init in async mode.
    let i2c_bus = i2c::master::I2c::new(
        peripherals.I2C0,
        i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
    ).expect("Init I2C bus").with_sda(peripherals.GPIO8).with_scl(peripherals.GPIO9).into_async();

    // We'll need to share the bus: 2 devices.
    // Make the bus static; init 2 shared buses
    static I2C_BUS: StaticCell<Mutex<NoopRawMutex, i2c::master::I2c<Async>>> = StaticCell::new();
    let i2c_shared_bus = I2C_BUS.init(Mutex::new(i2c_bus));

    // Create device handles for each peripheral
    let display_i2c = I2cDevice::new(i2c_shared_bus);
    let accelerometer_i2c = I2cDevice::new(i2c_shared_bus);




    // === Spawn Tasks === //
    spawner.spawn(task_accelerometer_jump_detection(accelerometer_i2c)).unwrap();




    // === Init Display === //

    // Init SSD1306 driver. Pass i2c bus to it.
    let display_iface = ssd1306::I2CDisplayInterface::new(display_i2c);
    let mut display = ssd1306::Ssd1306Async::new(
        display_iface,
        ssd1306::size::DisplaySize128x64,
        ssd1306::rotation::DisplayRotation::Rotate0,
    );
    display.init_with_addr_mode(ssd1306::command::AddrMode::Page).await.expect("Display init failed");


    // === Screen 1. Print ". . .". Display in "Terminal mode". === //

    // "Terminal Mode". Can only print text.
    let mut display = display.into_terminal_mode();

    // Clear the display first. Also resets the cursor to the top left corner.
    display.clear().await.expect("Clear");

    // Write ". . ."
    display.write_str("\n\n").await.expect("Text");
    for _ in 0..5 {
        Timer::after(Duration::from_millis(300)).await;
        display.write_str(" .").await.unwrap();
    }

    // Wait
    Timer::after(Duration::from_millis(300)).await;



    // === Screen 2. Print "No Internet". Display in "Graphics mode". === //

    // "Buffered Graphics Mode". Supports rich drawing features using `embedded_graphics`.
    let mut display = display.into_buffered_graphics_mode();
    display.init_with_addr_mode(ssd1306::command::AddrMode::Horizontal).await.expect("Display init failed");
    display.clear_buffer();  // clear the screen

    // Init text style
    let text_style = mono_font::MonoTextStyleBuilder::new().
        font(&mono_font::ascii::FONT_6X13).
        text_color(pixelcolor::BinaryColor::On).
        background_color(pixelcolor::BinaryColor::Off).
        build();

    // Print characters one by one
    let text = "No Internet.";
    for (i, c) in text.chars().enumerate() {
        let mut tmp = [0u8; 4];
        Text::new(
            c.encode_utf8(&mut tmp),
            Point::new(
                text_style.font.character_size.width as i32 * i as i32,
                3 * text_style.font.baseline as i32
            ),
            text_style
        ).draw(&mut display).expect("Print text");

        // Flush: write data to the display. Only then it gets updated.
        display.flush().await.unwrap();
        Timer::after(Duration::from_millis(200)).await;
    }

    // Clear screen
    Timer::after(Duration::from_secs(1)).await;
    display.clear_buffer();
    display.flush().await.unwrap();


    // === Game. Dino. === //

    // Prepare the Dino sprite
    // From array
    let _im_dino = ImageRaw::<pixelcolor::BinaryColor>::new(&DINO_SPRITE, DINO_SIZE.width);

    // From BMP
    let dino_l = tinybmp::Bmp::from_slice(include_bytes!("../../misc/dino-l.bmp")).unwrap();
    let dino_r = tinybmp::Bmp::from_slice(include_bytes!("../../misc/dino-r.bmp")).unwrap();
    let cactus = tinybmp::Bmp::from_slice(include_bytes!("../../misc/cactus.bmp")).unwrap();
    let cloud = tinybmp::Bmp::from_slice(include_bytes!("../../misc/cloud.bmp")).unwrap();
    use embedded_graphics::geometry::OriginDimensions; // adds: .size()

    // Sleep
    let now = Instant::now();
    let mut jump_in_progress: Option<Instant> = None;
    loop {
        // Clear display
        display.clear_buffer();

        // Dinosaur leg: left or right?
        let dino = if (now.elapsed().as_millis() / 400) % 2 == 0 {
            &dino_l
        } else {
            &dino_r
        };

        // Draw a cactus
        let x = display.size().width as i32 - (now.elapsed().as_millis() as u32/30 % (display.size().width + cactus.size().width)) as i32;
        let y = display.size().height as i32 - cactus.size().height as i32;
        let image = Image::new(&cactus, Point::new(x, y));
        image.draw(&mut display).unwrap();

        // Draw a cloud
        let x = display.size().width as i32 - (now.elapsed().as_millis() as u32/90 % (display.size().width + cloud.size().width)) as i32;
        let y = 0;
        let image = Image::new(&cloud, Point::new(x, y));
        image.draw(&mut display).unwrap();

        // Is the dino jumping?
        if jump_in_progress.is_none() &&
           let Some(jumped_at) = *LAST_JUMP_DETECTED.lock().await &&
           jumped_at.elapsed() < Duration::from_millis(200) {
                defmt::info!("Jump received: {}", jumped_at);
                // Use that instant, not the current one
                jump_in_progress = Some(jumped_at);
        }

        // Draw a dinosaur
        let mut y = display.size().height as i32 - dino.size().height as i32;
        if let Some(jump) = jump_in_progress {
            let t = (jump.elapsed().as_millis() / 100) as i32;
            let dy = 13 * t - t*t;
            if dy < 0 {
                jump_in_progress = None;
            } else {
                y -= dy;
            }
        }
        let image = Image::new(dino, Point::new(0, y));
        image.draw(&mut display).unwrap();

        // Draw
        display.flush().await.unwrap();

        // Sleep. Let others run.
        Timer::after(Duration::from_millis(30)).await;
    }
}




// Shared state: tells main() whether the user is jumping or not
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
};
static LAST_JUMP_DETECTED: Mutex<CriticalSectionRawMutex, Option<Instant>> = Mutex::new(None);

// Task: Accelerometer jump detection
#[embassy_executor::task]
async fn task_accelerometer_jump_detection(mut accelerometer_i2c: I2cDevice<'static, NoopRawMutex, I2c<'static, Async>>) -> ! {
    // Import the I2c trait. Otherwise the .write() method won't be accessible.
    use embedded_hal_async::i2c::I2c;  // .write()

    // === Init Accelerometer === //

    // Set up ADXL345
    const ADXL345_ADDR: u8 = 0x53;  // 0x53 if SDO is LOW, 0x1D if SDO is HIGH
    const REG_POWER_CTL: u8 = 0x2D;
    const REG_DATA_FORMAT: u8 = 0x31;
    const REG_DATAZ0: u8 = 0x36;  // Z-axis LSB
    const _REG_DATAZ1: u8 = 0x37;  // Z-axis MSB
    // Set measurement mode (disable standby)
    accelerometer_i2c.write(ADXL345_ADDR, &[REG_POWER_CTL, 0x08]).await.expect("ADXL power on");
    // Set data format: ±2g range, full resolution
    accelerometer_i2c.write(ADXL345_ADDR, &[REG_DATA_FORMAT, 0x08]).await.expect("ADXL set up");
    // Let it settle
    Timer::after_millis(10).await;

    // Keep checking the accelerometer: the user might be jumping.
    let mut last_jump_detected = Instant::now();
    loop {
        // Sacrifice some precision to keep the bus clean.
        Timer::after(Duration::from_millis(10)).await;

        // Don't detect jumps if a jump is already in progress.
        // This will also save us some Mutex locking. And some bus noise.
        // Alternatively, we could just sleep after detection. So much easier.
        if last_jump_detected.elapsed() < Duration::from_millis(100) {
            continue;
        }

        // Check the accelerometer. Is the user jumping?
        // Read two registers in one transaction.
        // This works because ADXL345 auto-increments the register address during multi-byte reads.
        let mut data = [0u8; 2];
        accelerometer_i2c.write_read(ADXL345_ADDR, &[REG_DATAZ0], &mut data).await.unwrap();
        // Combine LSB and MSB into signed 16-bit value
        let raw = i16::from_le_bytes(data);
        // Convert the raw value into something readable.

        // Jumping: <50 or >500
        // This is the raw value. We could've converted it into real acceleration reading.
        if raw < 50 || raw > 500 {
            // Remember the jump detected time.
            // The main() thread can decide whether it's recent enough.
            last_jump_detected = Instant::now();
            defmt::info!("Jump detected!");
            *LAST_JUMP_DETECTED.lock().await = Some(last_jump_detected);
        }
    }
}






// 'dino-small-inv', WxH Pixel = 20 x 22 px
const DINO_SIZE: Size = Size::new(20, 22);
#[rustfmt::skip]
const DINO_SPRITE: [u8; 66] = [
    // Generated with: https://implferris.github.io/image2bytes/
    // Uses bit array
	0x00, 0x1f, 0xe0, 0x00, 0x3f, 0xf0, 0x00, 0x37, 0xf0, 0x00, 0x3f, 0xf0, 0x00, 0x3f, 0xf0, 0x00,
	0x3f, 0xf0, 0x00, 0x3e, 0x00, 0x00, 0x3f, 0xc0, 0x80, 0x7c, 0x00, 0x80, 0xfc, 0x00, 0xc3, 0xff,
	0x00, 0xe7, 0xfd, 0x00, 0xff, 0xfc, 0x00, 0xff, 0xfc, 0x00, 0x7f, 0xf8, 0x00, 0x3f, 0xf8, 0x00,
	0x1f, 0xf0, 0x00, 0x0f, 0xe0, 0x00, 0x07, 0x60, 0x00, 0x06, 0x20, 0x00, 0x04, 0x20, 0x00, 0x06,
	0x30, 0x00
];


```





# tutorials/b11-adc-thermistor/src/bin


# tutorials/b11-adc-thermistor/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use {esp_backtrace as _, esp_println as _};
esp_bootloader_esp_idf::esp_app_desc!();
use defmt;

// Non-blocking
use nb;

// Math for no_std
use libm;

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    analog::adc,
    main,
};


#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default()
        .with_cpu_clock(CpuClock::max()));

    // ADC Input
    // Attenuation: -11dB. Gives us a safe range of 0..3.9V = 1100 * 10 ^ (11/20)
    let ldr_pin = peripherals.GPIO4;
    let mut adc_config = adc::AdcConfig::new();
    let mut pin = adc_config.enable_pin(ldr_pin, adc::Attenuation::_11dB);
    let mut adc1 = adc::Adc::new(peripherals.ADC1, adc_config);

    let delay = Delay::new();
    loop {
        let adc_value: u16 = match nb::block!(adc1.read_oneshot(&mut pin)) {
            Ok(result) => result,
            Err(err) => {
                defmt::error!("err={}", err);
                continue;
            }
        };
        let temperature = adc_value_to_temperature(adc_value, 11);
        defmt::info!("Temperature: {}", temperature);

        delay.delay_millis(100);
    }
}



// Math section

// Our voltage divider circuit
const ADC_VREF: f64 = 2.5; // ESP32 ADC Reference Voltage at the given attenuation (see docs)
const VDV_VDD: f64 = 3.3;   // Input voltage on the divider
const VDV_R1: f64 = 10_000.0;  // Second resistor in the voltage divider

// ADC resolution: 12 bits
// I.e. it maps voltages into 0..4095
const ADC_MAX: i32 = 2 << 12;  // 4095

// The typical B value for the NTC 103 thermistor is 3950.
// The reference temperature is usually 25°C and 10kΩ
const TRM_B_VALUE: f64 = 3950.0;
const TRM_REF_TEMP: f64 = 25.0; // Reference temperature 25°C
const TRM_REF_RES: f64 = 10_000.0; // Thermistor resistance at the Reference Temperature(25°C)


// Calc: measured resistance for RT:
//                         ┌──────────────── GPIO4
//   V_DD ──────[ R_1 ]────┴───[ R_T ]────── GND
//
//   $V_{measured} = V_{ref} * {adc\_value} / {ADC\_MAX}$
//   $R_T = R_1 / (V_{DD}/V_{measured} - 1)$
fn adc_value_to_temperature(adc_value: u16, att_db: u8) -> f64 {
    let V_measured: f64 = ADC_VREF * adc_value as f64 / ADC_MAX as f64;
    let R_thermistor: f64 = VDV_R1 / (VDV_VDD / V_measured - 1.0);
    // defmt::info!("adc_value={}", adc_value);
    // defmt::info!("V_measured={}", V_measured);
    // defmt::info!("R_thermistor={}", R_thermistor);

    // $1/T = 1/T_0 + (1/B) * ln(R/R_0)$
    const K: f64 = 273.15; // Kelvin
    let inv_t = 1.0/(TRM_REF_TEMP + K) + (1.0/TRM_B_VALUE) * libm::log(R_thermistor / TRM_REF_RES);
    return 1.0/inv_t - K;
}
```





# tutorials/b12-spi-sdcard/src/bin


# tutorials/b12-spi-sdcard/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use {esp_backtrace as _, esp_println as _};
// use {esp_println as _};
esp_bootloader_esp_idf::esp_app_desc!();
use defmt::info;
use core::fmt::Debug;

use esp_hal::{
    time::Rate,
    clock::CpuClock,
    delay::Delay,
    gpio::{Output, Level, OutputConfig},
    spi::{
        Mode as SpiMode,
        master::{Spi, Config as SpiConfig},
    },
    main,
};

use embedded_hal_bus::{
    spi::ExclusiveDevice,
};
use embedded_sdmmc::{
    SdCard, VolumeIdx, VolumeManager
};


// The sdmmc driver needs a time source to get the current time to handle ctime/mtime for files
#[derive(Default)]
pub struct DummyTimesource();
impl embedded_sdmmc::TimeSource for DummyTimesource {
    // In theory you could use the RTC of the rp2040 here,
    // if you had any external time synchronizing device.
    // See: https://esp32.implrust.com/sdcard/write-sdcard.html
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}



#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    let delay = Delay::new();

    // Init SPI bus
    let spi_bus = Spi::new(
        peripherals.SPI2,
        SpiConfig::default()
            // SD Cards require intial SCK=100 kHz .. 400 kHz
            .with_frequency(Rate::from_khz(400))
            .with_mode(SpiMode::_0),
        ).unwrap()
        .with_sck(peripherals.GPIO4)
        .with_miso(peripherals.GPIO5)
        .with_mosi(peripherals.GPIO6)
        ;

    // Init time source for SDcard file mtimes
    let sd_timer = DummyTimesource{};

    // Device, Driver, with its Chip Select
    // NOTE: that `SdCard` defers initialization until it's first used.
    // NOTE: CS is active-LOW, so we start with HIGH: not selected
    let cs = Output::new(peripherals.GPIO7, Level::High, OutputConfig::default());
    let spi_dev = ExclusiveDevice::new(spi_bus, cs, delay).unwrap();
    let sdcard = SdCard::new(spi_dev, delay);

    // Get SD card info
    let sd_size = sdcard.num_bytes().expect("Get SD card size");
    let sd_type = sdcard.get_card_type().expect("Get SD card type");
    defmt::info!("SD size: {}", sd_size);
    defmt::info!("SD type: {}", sd_type);

    // Speed up after init
    // TODO: (reconfigure SPI to 10-20 MHz here)
    sdcard.spi(|spi| spi.bus_mut().apply_config(
        &SpiConfig::default().with_frequency(Rate::from_mhz(12))
    )).expect("Switch to higher speeds");

    // Volume Manager: find a partition: volume #0
    let volume_mgr = VolumeManager::new(sdcard, sd_timer);
    let volume0 = volume_mgr.open_volume(VolumeIdx(0)).unwrap();
    let root_dir = volume0.open_root_dir().unwrap();

    // Write to file
    let mut my_file = root_dir.open_file_in_dir(
        "FERRIS.TXT",
        embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
    ).expect("Open file for writing");
    let line = "Hello, Ferris!";
    match my_file.write(line.as_bytes()) {
        Ok(()) => my_file.flush().expect("Write to file"),
        Err(e) => defmt::error!("Failed to write to file")
    }
    my_file.close().expect("Close file");

    // Read the file
    let mut my_file = root_dir
        .open_file_in_dir("FERRIS.TXT", embedded_sdmmc::Mode::ReadOnly)
        .expect("Open file for reading");
    while !my_file.is_eof() {
        let mut buffer = [0u8; 32];
        if let Ok(n) = my_file.read(&mut buffer) {
            for b in &buffer[..n] {
                defmt::info!("{}", *b as char);
            }
        }
    }

    loop {
        delay.delay_millis(100);
    }
}

```





# tutorials/b15-bluetooth/src/bin


# tutorials/b15-bluetooth/src/bin/main.rs

```rust
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

```





# tutorials/b15-bluetooth/src


# tutorials/b15-bluetooth/src/ble.rs

```rust
use anyhow::{Result};

use embassy_sync;
use embassy_futures::{
    join::join,
    select::select,
};
use embassy_time::Timer;

use trouble_host::prelude::*;
use esp_radio::{
    ble::controller::BleConnector,
};
use bt_hci::{
    FromHciBytesError, controller::ExternalController, transport::Transport
};


// GATT Server definition
#[gatt_server]
struct Server {
    // One service
    sensor_service: SensorService,
}

/// Battery service
#[gatt_service(uuid = "a9c81b72-0f7a-4c59-b0a8-425e3bcf0a0e")]
struct SensorService {
    // Characteristrics

    // Read-only number
    #[characteristic(uuid = "13c0ef83-09bd-4767-97cb-ee46224ae6db", read, notify)]
    sensor_data: u8,

    // Writable bool
    #[characteristic(uuid = "c79b2ca7-f39d-4060-8168-816fa26737b7", write, read)]
    sensor_settings: bool,
}

// Max number of simultaneous connections to this server
const CONNECTIONS_MAX: usize = 1;
// Max number of communication channels (?)
const L2CAP_CHANNELS_MAX: usize = 1;
// The number of commands that may wait for responses
const BT_CONTROLLER_SLOTS: usize = 20;

// We'll create a BLE server:
// * GATT service
// * two characteristics: one rw, one ro
pub async fn run<'d>(transport: BleConnector<'d>)
    // where T: Transport
    -> Result<()>
{
    // Init Bluetooth. HCI controller. Resources (mem). Stack.
    // Generated code.
    let ble_controller = ExternalController::<_, BT_CONTROLLER_SLOTS>::new(transport);
    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> = HostResources::new();
    let stack = trouble_host::new(ble_controller, &mut resources);

    // Set random BT address: 48 bit. Like a MAC address.
    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    let stack = stack.set_random_address(address);

    // Build stack
    let Host {
        // Get the "peripheral" part.
        mut peripheral,
        runner,
        // We don't need the  "central" part
        // mut central,
        ..
    } = stack.build();

    // Start advertising the GATT service
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        // Peripheral device name
        name: "iBeacon",
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    })).unwrap();

    // Infinite loop. Run both tasks.
    // join() joins the results of two futures
    let _ = join(
        // Task: The BLE stack task: keeps the stack running
        ble_task(runner),
        // Task: Our application logic
        async {
        loop {
            // Advertise: accept incoming connections and handle them.
            // It's like listen() + accept()
            match advertise("impl Rust", &mut peripheral, &server).await {
                // Connection established
                Ok(conn) => {
                    // Handle GATT events: GattEvent::{Read, Write, Other}
                    let a = gatt_events_task(&server, &conn);
                    // Run our application-specific logic during connection
                    let b = custom_task(&server, &conn, &stack);
                    // Run both tasks concurrently until one task ends (usually because of a disconnect)
                    // select() waits for one of two futures to complete
                    select(a, b).await;
                }
                // Connection failed
                Err(e) => {
                    let e = defmt::Debug2Format(&e);
                    panic!("BLE host failed: {:?}", e);
                }
            }
        }
    })
    .await;

    Ok(())
}

// BLE Stack Task
async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            let e = defmt::Debug2Format(&e);
            panic!("BLE core task error: {:?}", e);
        }
    }
}


// Advertise the service: announces its presense to nearby devices
// It broadcasts the device name, available services, and capabilities.
async fn advertise<'values, 'server, C: Controller>(
    // Device name
    name: &'values str,
    // BT Peripheral
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    // GATT server
    server: &'server Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    // Encode advertise data
    let mut advertiser_data = [0; 31];  // bufferw
    let len = AdStructure::encode_slice(
        &[
            // Discoverable + Classic not available
            // BR_EDR: basic rate (BR), enhanced data rate (EDR)
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;

    // Advertise: peripheral.advertise()
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            // Advertisement.
            // Connectable: device can connect
            // Scannable: devices can request additional information (though we provide empty `scan_data` here)
            // Undirected: invite-all, not a specific device
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..len],
                scan_data: &[],
            },
        )
        .await?;

    // Wait for connection, keep advertising
    defmt::info!("BLE: advertising...");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    defmt::info!("BLE: connection established");
    Ok(conn)
}


// Handle GATT events
async fn gatt_events_task<P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) -> Result<(), Error> {
    // Our read-only characteristic.
    // This was generated by the macro when we defined the the #[gatt_ser]
    let sensor_data = server.sensor_service.sensor_data;

    // Loop until there's a disconnected reason
    let reason = loop {
        // Get next event
        match conn.next().await {
            // Disconnected: quit loop
            GattConnectionEvent::Disconnected { reason } => break reason,
            // GATT Event: unwrap
            GattConnectionEvent::Gatt { event } => {
                // GATT Event
                match &event {
                    // Central device reads a characteristic.
                    GattEvent::Read(event) => {
                        // Which characteristic?
                        if event.handle() == sensor_data.handle {
                            // Retrieve the value from the server and log it
                            let value = server.get(&sensor_data);
                            match server.get(&sensor_data) {
                                Ok(value) => defmt::info!("[gatt] Read Event to Sensor Data Characteristic: {:?}", value),
                                Err(e) => defmt::error!("[gatt] Get server characteristic failed: {}", e),
                            }
                        }
                    }
                    // Central device writes a characteritic.
                    GattEvent::Write(event) => {
                        // Which characteristic?
                        if event.handle() == sensor_data.handle {
                            // Data written! We are kinda callback
                            defmt::info!("[gatt] Write Event to Sensor Data Characteristic: {:?}", event.data());
                        }
                    }
                    _ => {}
                };

                // This step is also performed at drop(), but writing it explicitly is necessary
                // in order to ensure reply is sent.
                match event.accept() {
                    Ok(reply) => reply.send().await,
                    Err(e) => defmt::warn!("[gatt] error sending response: {:?}", e),
                };
            }
            _ => {} // ignore other Gatt Connection Events
        }
    };

    defmt::info!("BLE: disconnected: {:?}", reason);
    Ok(())
}


// Example task
// - Notify the connected central of a counter value every 2 seconds
// - Read the RSSI value every 2 seconds
// - Stop when the connection is closed
async fn custom_task<C: Controller, P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    stack: &Stack<'_, C, P>,
) {
    // GATT service characteristic
    let sensor_data = server.sensor_service.sensor_data;

    // The counter
    let mut tick: u8 = 0;
    loop {
        // Increment
        tick = tick.wrapping_add(1);

        // Notify everyone listening
        defmt::info!("[custom_task] notifying connection of tick {}", tick);
        if sensor_data.notify(conn, &tick).await.is_err() {
            defmt::info!("[custom_task] error notifying connection");
            break;
        };

        // Read RSSI
        if let Ok(rssi) = conn.raw().rssi(stack).await {
            defmt::info!("[custom_task] RSSI: {:?}", rssi);
        } else {
            defmt::info!("[custom_task] error getting RSSI");
            break;
        };

        // Sleep
        Timer::after_secs(2).await;
    }
}

```





# tutorials/b16-bluetooth-scanner/src/bin


# tutorials/b16-bluetooth-scanner/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
esp_bootloader_esp_idf::esp_app_desc!();
use {esp_backtrace as _, esp_println as _};
extern crate alloc;

use esp_hal::{
    clock::CpuClock,
    timer::timg::TimerGroup,
    gpio,
};
use embassy_executor::Spawner;
use embassy_time::Timer;

// Bluetooth
use trouble_host::prelude::*;
use esp_radio::{
    ble::controller::BleConnector,
};

// Util
use static_cell::StaticCell;

// Our module
use b16_bluetooth_scanner::blscanner;


// BLE scanner with a buzzer.
// Produces more clicks as you approach a specific BLE device identified by MAC address.
// NOTE: BLE devices randomize their addresses! You can't rely on a MAC address. You'll need to pair.
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // RTOS
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    // GPIO Active Buzzer.
    // It's open-drain and active-LOW.
    let buzzer_pin = peripherals.GPIO4;
    let mut buzzer = gpio::Output::new(buzzer_pin, gpio::Level::High, gpio::OutputConfig::default().with_drive_mode(gpio::DriveMode::OpenDrain));

    // ESP Bluetooth. Radio. Transport.
    let radio = {
        static RADIO: StaticCell<esp_radio::Controller> = StaticCell::new();
        RADIO.init(esp_radio::init().expect("Init radio"))
    };
    let bluetooth = BleConnector::new(radio, peripherals.BT, Default::default()).expect("Init bluetooth");

    // Run BT tasks
    blscanner::run_tasks(&spawner, bluetooth, buzzer).await.expect("Run bluetooth");

    // Idle run
    loop {
        Timer::after_secs(1).await;
    }
}

```





# tutorials/b16-bluetooth-scanner/src


# tutorials/b16-bluetooth-scanner/src/blscanner.rs

```rust
use defmt;
use anyhow::Result;
use core::sync::atomic::{AtomicU8, Ordering};
use core::cell::RefCell;
use heapless::Deque;


use esp_hal::gpio;
use esp_radio::{
    ble::controller::BleConnector,
};
use trouble_host::prelude::*;

use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};
use embassy_futures::join::join;

// The specific BT device to find.
// Buzzer will click louder as you approach the device and the signal gets better.
//
// NOTE: BT addresses are Little-Endian so written BACKWARDS!
const TARGET: &[u8] = &[0x41, 0xFE, 0x83, 0x1C, 0x8F, 0x52];

// Max number of simultaneous connections to this server
const CONNECTIONS_MAX: usize = 1;
// Max number of communication channels (?)
const L2CAP_CHANNELS_MAX: usize = 1;
// The number of commands that may wait for responses
const BT_CONTROLLER_SLOTS: usize = 20;

pub async fn run_tasks(
    spawner: &Spawner,
    bt_transport: BleConnector<'static>,
    buzzer: gpio::Output<'static>
) -> Result<()> {
    // Init Bluetooth. HCI controller. Resources (mem). Stack.
    // Generated code.
    let ble_controller = ExternalController::<_, BT_CONTROLLER_SLOTS>::new(bt_transport);
    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> = HostResources::new();
    let stack = trouble_host::new(ble_controller, &mut resources);

    // Set random BT address: 48 bit. Like a MAC address.
    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    let stack = stack.set_random_address(address);

    // Build stack
    let Host { central, mut runner, ..  } = stack.build();

    // Finally: spawn tasks
    spawner.spawn(task_buzzer(buzzer))?;
    spawner.spawn(task_scan_bluetooth())?;

    // Prepare scanner
    // Requires: $ cargo add trouble-host --features scan
    let mut scanner = Scanner::new(central);
    let scan_config = ScanConfig{
        active: true,
        phys: PhySet::M1,
        interval: Duration::from_secs(3),
        window: Duration::from_secs(3),
        ..Default::default()
    };

    // BT events handler
    let handler = Printer { seen: RefCell::new(Deque::new()) };
    let handler = BleFoundDeviceHandler {};


    // Scan continuously: join() two tasks
    // - run_with_handler() to process BT events: i.e. found device
    // - scan, repeat, scan
    let _ = join(
        runner.run_with_handler(&handler),
        async {
            loop {
                // Scan. Then wait for the exact scan time.
                defmt::info!("Scan");
                let mut session = scanner.scan(&scan_config).await.unwrap();
                Timer::after(scan_config.interval).await;
            }
        }
    ).await;

    Ok(())
}

// Task: Bluetooth Scanner
#[embassy_executor::task]
pub async fn task_scan_bluetooth() -> ! {
    loop {
        Timer::after_secs(2).await;
    }
}

// How close is the target? 1..255.
// Value=0 means "not found".
// Defines the frequency of clicks.
static PROXIMITY: AtomicU8 = AtomicU8::new(0);

// Task: Buzzer.
// Produces period "beeps" as you approach the target.
#[embassy_executor::task]
pub async fn task_buzzer(mut buzzer: gpio::Output<'static>) -> ! {
    loop {
        // Delay between clicks
        let prox = PROXIMITY.load(Ordering::Relaxed);
        let delay = Duration::from_millis(4*(prox as u64));

        // Not found?
        if prox == 0 {
            Timer::after_secs(1).await;
            continue;
        }

        // Click
        buzzer.toggle();
        Timer::after_micros(750).await;
        buzzer.toggle();

        // Delay
        Timer::after(delay).await;
    }
}


// BT event handler for found devices.
// The original handler: prints only newly found devices; uses Dequeue for deduplication.
struct Printer {
    seen: RefCell<Deque<BdAddr, 128>>,
}
impl EventHandler for Printer {
    fn on_adv_reports(&self, mut it: LeAdvReportsIter<'_>) {
        let mut seen = self.seen.borrow_mut();
        while let Some(Ok(report)) = it.next() {
            if seen.iter().find(|b| b.raw() == report.addr.raw()).is_none() {
                defmt::info!("discovered: {:?} rssi={:?}", report.addr, report.rssi);
                if seen.is_full() {
                    seen.pop_front();
                }
                seen.push_back(report.addr).unwrap();
            }
        }
    }
}


// BT event handler for found devices.
// Out handler updates the PROXIMITY value
struct BleFoundDeviceHandler {}
impl EventHandler for BleFoundDeviceHandler {
    fn on_adv_reports(&self, mut it: LeAdvReportsIter<'_>) {
        while let Some(Ok(report)) = it.next() {
            if report.addr.raw() == TARGET {
                let prox = rssi_to_proximity(report.rssi);
                PROXIMITY.store(prox, Ordering::Relaxed);
                defmt::info!("TARGET: {:?} rssi={:?} prox={:?}", report.addr, report.rssi, prox);
            } else {
                defmt::info!("another: {:?} rssi={:?}", report.addr, report.rssi);
            }
        }
    }
}

// RSSI -> Proximity. Good RSSI: >-30 Bad RSSI: <-90
// Map to: 1..255. Logarithmic.
fn rssi_to_proximity(rssi: i8) -> u8 {
    let clamped = rssi.clamp(-90, -30);

    // Exponential decay: closer signals are much stronger
    // Map [-30, -90] with exponential curve
    let normalized = (-30 - clamped) as f32 / 60.0; // [0.0, 1.0]

    // Apply exponential curve (adjust exponent to taste)
    use libm;
    let curved = libm::powf(normalized, 2.5); // More aggressive curve

    let proximity = (curved * 254.0) + 1.0;
    proximity as u8
}

```





# tutorials/b16-bluetooth-scanner


# tutorials/b16-bluetooth-scanner/.gitignore

```
# will have compiled files and executables
debug/
target/

# Editor configuration
.vscode/
.zed/
.helix/
.nvim.lua

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





# tutorials/b17-lcd-i2c/src/bin


# tutorials/b17-lcd-i2c/src/bin/main.rs

```rust
#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
#![deny(clippy::large_stack_frames)]
esp_bootloader_esp_idf::esp_app_desc!();
use {esp_backtrace as _, esp_println as _};
use defmt;

use esp_hal::{
    clock::CpuClock,
    time::{Duration, Instant, Rate},
    delay::Delay,
    i2c::master::{I2c, Config as I2cConfig},
    main,
};

// HD44780 Driver
// use hd44780_driver::{
//     HD44780,
//     // memory_map::MemoryMap1602,
//     // setup::DisplayOptionsI2C,
// };
use hd44780_driver::{
    setup::DisplayOptionsI2C,
    HD44780,
};

#[allow(clippy::large_stack_frames)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut i2c = I2c::new(
            peripherals.I2C0,
            I2cConfig::default().with_frequency(Rate::from_khz(400)),
        ).expect("i2c bus init")
        .with_sda(peripherals.GPIO8)
        .with_scl(peripherals.GPIO9);

    // Scan I2C bus
    // - I2C uses 7-bit addresses (0x00-0x7F)
    // - Skip 0x00-0x02 (reserved for special protocols)
    // - Skip 0x78-0x7F (10-bit addressing and reserved)
    for addr in 0x03..=0x77 {
        match i2c.write(addr, &[]) {
            Ok(_) => {
                defmt::info!("Device found at 0x{:02X}", addr);
            }
            Err(_) => {
                // No device here, NACK received
                // NACK is assumed if no device pulls the line LOW.
            }
        }
    }

    // Init display.
    // I2C address = 0x27
    let i2c_address = 0x27;
    let mut delay = Delay::new();
    let mut options = DisplayOptionsI2C::new(
        // Memory map: 16x2 characters
        hd44780_driver::memory_map::MemoryMap1602::new()
    ).with_i2c_bus(i2c, i2c_address);

    let mut display = loop {
		match HD44780::new(options, &mut delay) {
			Err((options_back, error)) => {
				defmt::error!("Error creating LCD Driver: {:?}", defmt::Debug2Format(&error));
				options = options_back;
				delay.delay_millis(500);
				// try again
			}
			Ok(display) => break display,
		}
	};

    // Unshift display and set cursor to 0
    // Then clear existing characters
	display.reset(&mut delay).unwrap();
    display.clear(&mut delay).unwrap();
	display.write_str("31337", &mut delay).unwrap();

    loop {
        delay.delay_millis(500);
    }
}

```

