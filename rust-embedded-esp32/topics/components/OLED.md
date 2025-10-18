# OLED Display

OLED: Organic Light-Emitting Diode.

OLED displays consume less power and offer better performance than LCDs
since they don't require a backlight.

## I2C

For I2C, two GPIO pins need to be configured as SDA (Serial Data) and SCL (Serial Clock).
On ESP32-C3, only GPIO 8 and 9 can be used for I2C.
So:

* GPIO8 → SDA
* GPIO9 → SCL
* 3.3V → VCC
* GND → GND

It doesn't require any pull-up resistors because they're included on the board itself.

## GDDRAM

In the datasheet, the 128 columns are referred to as segments,
while the 64 rows are called commons.

The OLED display's pixels are arranged in a page structure within GDDRAM (Graphics Display DRAM).
GDDRAM is divided into 8 pages (From Page 0 to Page 7), each consisting of 128 columns (segments) and 8 rows (commons).

A segment is 8 bits of data (one byte), with each bit representing a single pixel.
When writing data, you will write an entire segment, meaning the entire byte is written at once.

We can, of course, re-map both segments and commons through software.

Use crates:

* [`ssd1306`](https://docs.rs/ssd1306/latest/ssd1306/): a driver interface for the SSD1306 monochrome OLED display,
  supporting both I2C and SPI through the [`display_interface`](https://docs.rs/display-interface/latest/display_interface/) crate.
  It also has async support that you have to enable through the feature flag.
* [`embedded_graphics`](https://docs.rs/embedded-graphics/latest/embedded_graphics/):
  a 2D graphics library that is focused on memory constrained embedded devices.
  Its goal is to draw graphics without using any buffers and pre-allocated memory.
  This is achieved with an iterator-based approach.w

```rust
// I2C. Init in async mode.
let i2c_bus = i2c::master::I2c::new(
    peripherals.I2C0,
    i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
).expect("Init I2C bus").with_sda(peripherals.GPIO8).with_scl(peripherals.GPIO9).into_async();

// Init SSD1306 driver. Pass i2c bus to it.
// Use a "buffered graphics" mode, supporting embedded-graphics.
let display_iface = ssd1306::I2CDisplayInterface::new(i2c_bus);
let mut display = ssd1306::Ssd1306Async::new(
    display_iface,
    ssd1306::size::DisplaySize128x64,
    ssd1306::rotation::DisplayRotation::Rotate0,
).into_buffered_graphics_mode();
display.init_with_addr_mode(ssd1306::command::AddrMode::Page).await.expect("Display init failed");
```
