Display: TFT SPI
================

TFT LCD: a type of LCD that uses *thin-film transistor* technology.

As of 2024, TFT LCDs are still dominant, but compete with OLED for high brightness and high resolution displays, and compete with electronic paper for low power displays.

Types:

* Twisted nematic (TN): oldest and cheapest. Fast response and less smearing — but poor color reproduction and limited viewing angles. Technology: liquid crystals twist 90° to let light pass.
* In-plane switching (IPS): great colors and viewing angles. Technology: crystal molecules move parallel to the panel plane instead of perpendicular to it.
* VA: better than TN but cheaper than IPS

# Interfaces

Interfaces for TFT panels:

* SPI: Serial interface: fewer wires but slower
* Parallel interface: faster but needs more pins (an 8-bit or 16-bit data bus plus control signals;. Protocols: 8080, 6800).
* RGB (TTL) interface: even faster, good for bigger displays — but requires a special controller to work

For a small display, the "slowness" of SPI is relative:
with 40MHz SPI you get ~5MB/s, enough for decent frame rates on 320x240.

## Controller
Every TFT display has a controller chip that takes care of drawing pixels on the screen.
In our case, the display uses the **ILI9341** chip.
We send commands to this chip, and it handles tasks like updating the screen, drawing shapes, filling colors, and more.

The ILI9341 controller is most commonly used in 2.4" and 2.8" TFT displays, and sometimes in 3.2" displays.
If your screen uses the same ILI9341 chip, it will work with the same code and examples in this tutorial.

You can also use displays with other driver chips (like **ST7735**, **ILI9225**, **ILI9486**),
but those will need different Rust crates or setup instructions.
So, it's important to check the controller chip used in your display before buying, to make sure it's compatible with the tutorial.

## Touch

Some TFT displays also come with a resistive touchscreen on top of the screen.
In these displays, touch functionality is handled by a separate chip called **XPT2046**.

This chip reads the X and Y position when you touch the screen and sends that data over SPI.
It works independently from the display driver (ILI9341).

