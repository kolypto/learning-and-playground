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

sometimes in the "pinout" image you'll see GPIx: "x" replacing the "O" means the pin can't be used as an output.

## Drive Mode: Push/Pull vs Open Drain

Push-pull GPIOs actively drive a pin both HIGH and LOW:
i.e. there will be voltage when HIGH and it will be grounded when LOW.

Open drain pins can only actively drive low, requiring an external pull-up resistor for a high state.
That is, you use open drain only when you have an external pullup.

* Pull-up resistor: GPIO is connected to a switch (buffer, transistor base, ...) and also to VCC through a resistor.
  This makes it an active-LOW.
* Pull-down resistor: GPIO is connected to a switch (buffer, transistor base, ...) and also to GND through a resistor.
  This makes it an active-HIGH.


Open-drain: pin either disconnected and floating, or active low. There is no high. This has a consequence of the pin high voltage not being set at all, it is set externally through pull-up resistors. Which means you can set high voltage to pretty much any voltage within pin spec, but you can’t drive anything directly from the pin.

Open-drain is usually used with buses: e.g. I2C required it.

