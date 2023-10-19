 
# Embedded Rust

Reading:

* [Embeeded Discovery Book](https://docs.rust-embedded.org/discovery/): small fun projects to teach you bare metal programming.
* [The Embedded Rust Book](https://doc.rust-lang.org/stable/embedded-book/): if you are familiar with embedded development
* [OS Tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials): learn to write an embedded OS in Rust! On Pi.
* [Embedonomicon](https://docs.rust-embedded.org/embedonomicon/): a deep dive into the implementation of the foundational crates: linker, symbols, and ABIs.

More resources:

* [Awesome Embedded Rust](https://github.com/rust-embedded/awesome-embedded-rust): curated list of libraries and teaching materials







# Introduction


## Libstd
The standard library contains primitives to interact with OS systems: FS, network, memory, threads, etc.
In a bare-metal environment, no code has been loaded before your program, so there's no OS abstractions
and no POSIX that the standard library depends upon.

To prevent Rust from loading the standard library, use `#![no_std]`. It's a crate-level attribute

The missing libstd also provides a runtime: takes care of setting up stack overflow protection,
processes command-line arguments, spawns the main thread before a program's `main` is invoked, etc.
This runtime also won't be available.

The platform-agnostic parts of the standard library are available through [libcore](https://doc.rust-lang.org/core/).
It also excludes things that are not always desirable in an embedded environment, like the memory allocator:
use crates of your choice.

The `libcore` contains:
APIs for language primitives (floats, strings, slices, etc),
APIs that expose processor features like "atomic" operations and SIMD instructions,
etc.

However, it lacks APIs for anything that involves platform integration:
because it can be used for any kind of bootstrapping (stage 0) code like bootloaders, firmware, or kernels.



## Tooling

Install:

* `rustup`: installs Rust and tooling
* [`cargo-generate`](https://github.com/cargo-generate/cargo-generate): a cargo subcommand to generate projects from templates. Alternatively, clone a git repo.
* `cargo-binutils`: tools for LLVM use to inspect binaries: `objdump`, `nm`, `size`
* `qemu-system-arm`: emulate ARM systems locally, run programs without having any hardware with you!
* GDB: you may not always have the luxury to log stuff to the host console.
  Also, LLDB doesn't yet support `load` that uploads the program to the target hardware.
  So, currently, GDB is recommended.
* OpenOCD/ESPtool: GDB isn't able to communicate directly with the hardware: it needs a translator.
  OpenOCD translates between GDB protocol and ST-Link's USB protocol. It knows to to read/write flash.
  It also knows how to interact with ARM CoreSight debug peripheral,
  which interacts with memory-mapped registers allow to breakpoint/watchpoint, read CPU registers, continue, etc.

Also you might want to add:

* `cargo-embed`: cargo-embed is the big brother of `cargo-flash`.
  It can flash a target, and it can also open an RTT terminal as well as a GDB server.
  Installed as a part of `probe-rs` tools.
* `minicom` to open a terminal with a USB-connected device

Install:

```console
$ sudo apt install cargo-binutils qemu-system-arm gdb-multiarch
$ cargo install cargo-generate
$ cargo install probe-rs --features cli,ftdi
$ sudo apt install esptool stm32flash openocd
```

alternatively, using rustup:

```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ cargo install cargo-binutils cargo-generate
$ rustup component add llvm-tools-preview
```

To use ST-Link without root privileges, you may need to create a udev rule:
[more info here](https://doc.rust-lang.org/stable/embedded-book/intro/install/linux.html):

```udev
# STM32F3DISCOVERY rev A/B - ST-LINK/V2
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", TAG+="uaccess"

# STM32F3DISCOVERY rev C+ - ST-LINK/V2-1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", TAG+="uaccess"

# CMSIS-DAP for microbit
SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", ATTR{idProduct}=="0204", MODE:="666"
```




Test that it works: with ST-Link: one of these:

```console
$ openocd -f interface/stlink.cfg -f target/stm32f3x.cfg
$ openocd -f interface/stlink-v2.cfg -f target/stm32f3x.cfg
$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
...
Info : Target voltage: 2.919881
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

if you don't have the "breakpoints" line, use a different config file.







## Enable Target Cross-Compilation

By default, Rust only supports native compilation.

Check the list of targets:

```console
$ rustc --print target-list
thumbv4t-none-eabi
thumbv5te-none-eabi
thumbv6m-none-eabi
thumbv7a-pc-windows-msvc
thumbv7a-uwp-windows-msvc
thumbv7em-none-eabi
thumbv7em-none-eabihf
thumbv7m-none-eabi
thumbv7neon-linux-androideabi
thumbv7neon-unknown-linux-gnueabihf
thumbv7neon-unknown-linux-musleabihf
thumbv8m.base-none-eabi
thumbv8m.main-none-eabi
thumbv8m.main-none-eabihf
...
riscv32imc-esp-espidf
...
```

This is how architectures are added:

```console
$ rustup target add thumbv6m-none-eabi
```




## Terminology

* PAC: Peripheral Access Crate. Provides a safe-ish direct interface to the peripherals of the chip.
  Normally you only deal with PACs if the higher level doesn't fulfil your needs.
* HAL: Hardware Abstraction Layer. It builds upon the chip's PAC and provides an abstraction.
  You can use the chip without knowing all the special behavior of the chip.
* BSP: Board Support Crate. Abstracts a whole board away at once, with all its sensors, leds, etc.
  Quite often, you will work with the HAL, and get drivers for your sensors from crates.io.

The central piece: (`embedded-hal`)[https://crates.io/crates/embedded-hal]: provides a set of traits
that describe behavior common to specific peripherals. These are common interfaces.
Drivers that are written in such a way are called platform agnostic. Most drivers are.

