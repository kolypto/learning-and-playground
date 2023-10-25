# Rust on ESP32 with `std`

Generate a project:

```console
$ cargo generate esp-rs/esp-idf-template cargo
$ cd <project-folder>
$ rustup override set esp
```

* choose "STD support = yes"
* when choosing the IDF version, note that not all chips support it.

Recommended: set the toolchain directory to "global":
otherwise, each new project will have its own instance of the toolchain and eat up disk space:

*   Add this to `.cargo/config.toml`:

    ```toml
    [env]
    ESP_IDF_TOOLS_INSTALL_DIR = { value = "global" } # add this line
    ```

*   Add this to `rust-toolchain.toml`:

    ```toml
    [toolchain]
    channel = "nightly-2023-02-28" # change this line
    ```

We'll use `toml_cfg`. All add `anyhow` to be used in the build script:

```console
$ cargo add toml_cfg anyhow esp-idf-hal embedded-svc
$ cargo add esp-idf-sys --features=binstart
$ cargo add --build toml_cfg anyhow
```




Further reading for ESP32 `std` programming:

* [HTTPS Server](https://esp-rs.github.io/std-training/03_4_http_server.html) and [code](https://github.com/esp-rs/std-training/tree/main/intro/http-server)
* [MQTT Client](https://esp-rs.github.io/std-training/03_5_0_mqtt.html) and [code](https://github.com/esp-rs/std-training/tree/main/intro/mqtt)
* [I2C, I/O, Sensors, Interrupts](https://esp-rs.github.io/std-training/04_0_advanced_workshop.html)

Further reading for ESP32:

* [Awesome ESP32](https://github.com/esp-rs/awesome-esp-rust)
