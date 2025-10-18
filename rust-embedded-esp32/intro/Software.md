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
