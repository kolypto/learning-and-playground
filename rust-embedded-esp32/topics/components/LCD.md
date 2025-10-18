LCD (I2C)
=========

Device: Hitachi HD44780 compatible LCD

* Can display ASCII + up to 8 custom characters
* Variants: 16x2, 20x4, different backlight colors
* Some of them come with an integrated I2C interface adapter

Parallel Interface
-------------------

At its core, the HD44780 controller uses a parallel interface.
This is the native and most direct way to communicate with the LCD.
Many modules expose this interface directly through their 16-pin header.

How It Works
------------

Each character is a 5x8 pixel grid.
You don't have to control pixels: the controller maps ASCII characters to pixel grids.
It can store 8 custom characters as well.

It supports two data modes:

* 8-bit mode: data is sent as a full byte using all the data pins. Uses more wires but is faster.
* 4-bit mode: only the higher-order data bits are used, sending data in nibbles.

I2C Module
----------

Pins:

* VCC, GND: Power supply (5V)
* SDA, SCL: I2C data and clock lines

Potentiometer: use to adjust contrast if the text is not clear.

