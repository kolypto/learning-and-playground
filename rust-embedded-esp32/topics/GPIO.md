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
