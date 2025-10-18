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
