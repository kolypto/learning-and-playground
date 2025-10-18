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






