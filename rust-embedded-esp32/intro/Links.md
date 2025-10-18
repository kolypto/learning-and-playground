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
