# I2C

"I2C": Inter-Integrated Circuit: synchronous serial communication protocol.

Uses two lines: a data line (SDA) and a clock line (SCL).
Because a clock line is used to synchronize the communication, this is a *synchronous* protocol.


The "controller" is the device that starts and drives the communication.

Several devices, both controllers and targets, can be connected to the same bus.

A controller communicates with a device by first broadcasting its address to the bus.
This address can be 7bits or 10bits long.

No other device can use the bus until the controller stops the communication (!)

The clock determines how fast data can be exchanged:
usually 100 kHz (standard mode) or 400 kHz (fast mode)

Protocol:

1. Controller sends START
2. Controller broadcasts target address (7 or 10 bits) + 1 R/W bit. It's set to "WRITE" for "controller -> target" communication, and "READ" for "controller <- target" communication.
3. Target responds with ACK
4. repeat ( Send one byte + Respond with ACK )
5. Controller broadcasts STOP (or RESTART and go back to 2)

## ADXL345 Accelerometer

What I have here is an ADXL345 accelerometer with I2C, SPI, and two configurable "interrupt" pins to report tap/double-tap/falling.

It is configurable: has a number of writable registers that contain the settings. The registers also contain the readings.

In a sense, these sensors are very similar to the peripherals inside the microcontroller.
The difference is that their registers are not mapped into the microcontroller's memory:
instead, their registers have to be acessed via the I2C bus.

Some accelerometer modules also have a magnetometer: LSM303AGR , MPU-6050.

### Practice

First: find out the target address of the accelerometer.
With ADXL345, the address is `0x1D`, followed by the R/W bit:
this translates to `0x3B` (R) and `0x3A` (W).

The wiring is important:

* GPIO 21 -> SDA
* GPIO 22 -> SCL
* Power: ideally 2.5V, can do up to 3.6V
* `CS` must be connected to VDDI/O (VCC)
* `SDO` selects the address: HIGH -> `0x1D`, LOW -> `0x53`. If not, there will be no response.

Second: lots of I2C chips will have some sort of device identification register.
With this device ID register, we can verify that we are indeed talking to the device we expect:
in our case, `DEVID` (`0x00` register) contains `11100101`.

## SSD1780 Display

This is a 128x64 OLED display with I2C and SPI interfaces.

Its address is `011110` + `1/0`.

