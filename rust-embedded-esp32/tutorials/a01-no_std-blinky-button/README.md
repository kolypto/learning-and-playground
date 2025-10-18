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
