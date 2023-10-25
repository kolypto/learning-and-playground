# Rust on ESP32

This is for the ESP32 board.

Reading:

* [ESP-RS book](https://esp-rs.github.io/book/)
* [ESP-RS training](https://esp-rs.github.io/std-training/): writing apps with `std`
* [Awesome ESP Rust](https://github.com/esp-rs/awesome-esp-rust): a collection

In the [esp-rs](https://github.com/esp-rs/) organization:

* `esp-*` repositories are focused on `no_std` applications: e.g. `esp-hal`
* `esp-idf-*` are focused on `std` apps: e.g. `esp-idf-hal`

## `std` vs `no_std`

### `std` on ESP32

Unlike most other embedded platforms, Espressif supports the Rust standard library.
Most notably, this means you'll have arbitrary-sized collections like `Vec` or `HashMap` at your disposal, as well as generic heap storage using `Box`.

You're also free to spawn new threads, and use synchronization primitives like `Arc` and `Mutex` to safely share data between them.
Still, memory is a scarce resource on embedded systems, and so you need to take care not to run out of it - threads in particular can become rather expensive.

Espressif provides a C-based development framework: [ESP-IDF](https://github.com/espressif/esp-idf), which provides a [newlib](https://sourceware.org/newlib/) environment that has enough functionality to build the Rust `std` on top of it.

When using `std`, you have access to a lot of `ESP-IDF` features: threads, mutexes, collections, random numbers, sockets, etc.

Services like Wi-Fi, HTTP client/server, MQTT, OTA updates, logging etc. are exposed via Espressif's open source IoT Development Framework, ESP-IDF.
It is mostly written in C and as such is exposed to Rust in the canonical split crate style:

* the [esp-idf-sys](https://github.com/esp-rs/esp-idf-sys) crate provides the actual `unsafe` bindings to the IDF development framework that implements access to drivers, Wi-Fi, and more
* the higher-level [esp-idf-svc](https://github.com/esp-rs/esp-idf-svc) crate implements safe and comfortable Rust abstractions: it implements abstractions from [embedded-svc](https://github.com/esp-rs/embedded-svc): wi-fi, network, httpd, logging, etc
* [esp-idf-hal](https://github.com/esp-rs/esp-idf-hal): implements traits from `embedded-hal` and other traits using the `esp-idf` framework: analog/digital conversion, digital I/O pins, SPI communication, etc.

You might want to use the `std` when your app:
* requires rich functionality (network, file I/O, sockets, complex data structures)
* for portability: because the `std` crate provide APIs that can be used across different platforms
* rapid development


### `no_std` on ESP32

Using `no_std` may be more familiar to embedded Rust developers: it uses a subset of `std`: the `core` library.

See crates:

* [esp-hal](https://github.com/esp-rs/esp-hal):	Hardware abstraction layer
* [esp-pacs](https://github.com/esp-rs/esp-pacs):	Peripheral access crates
* [esp-wifi](https://github.com/esp-rs/esp-wifi):	Wi-Fi, BLE and [ESP-NOW](https://www.espressif.com/en/solutions/low-power-solutions/esp-now) support
* [esp-alloc](https://github.com/esp-rs/esp-alloc):	Simple heap allocator
* [esp-println](https://github.com/esp-rs/esp-println):	`print!`, `println!`
* [esp-backtrace](https://github.com/esp-rs/esp-backtrace):	Exception and panic handlers
* [esp-storage](https://github.com/esp-rs/esp-storage):	Embedded-storage traits to access unencrypted flash memory

You might want to use the `no_std` when:

* You need a small memory footprint
* Direct hardware control. Because `std` adds abstractions that make it harder to interact directly with the hardware
* Real-time constraints or time-critical applications: because `std` can introduce unpredictable delays and overhead
* Custom requirements: fine-grained control over the behavior of an application


## Preparation

Espressif SoCs are based on two different architectures: RISC-V and Xtensa.

* Modern chips: ESP32-C/H/P are based on RISC-V.
* Older chips: ESP32, ESP32-S use Tensilica Xtensa.

For ESP32-C2, C2, C6, H2, P4:
[Tools for RISC-V Targets only](https://esp-rs.github.io/book/installation/riscv.html).

For all chips (Xtensa *and* RISC-V):

* install `espup`: simplifies installing and maintaining the components required to build
* Run `espup install` to install the toolchain: Rust fork, nightly toolchain, LLVM fork, GCC toolchain

Instead of installing, you can use a Docker image: [espressif/idf-rust
](https://hub.docker.com/r/espressif/idf-rust/tags):

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

## Start a project

Generate a project:

```console
$ cargo install cargo-generate
$ cargo generate esp-rs/esp-template
```

Questions from `cargo generate`:

* Which MCU? `esp32` with Xtensa architecture

Now build and flash:

```console
$ cargo build
$ cargo run
```

You can use `cargo run` because of `.cargo/config.toml`, which configures the build target and the runner:

```toml
[target.xtensa-esp32-none-elf]
runner = "espflash flash --monitor"
```



## VSCode

Rust analyzer can behave strangely without `std`.
Add this to `settings.json`:

```json
{
  "rust-analyzer.checkOnSave.allTargets": false
}
```

