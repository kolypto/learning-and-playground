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

We'll use `toml_cfg`:

```console
$ cargo add toml_cfg
```
