## Serial port

"Serial communication" is asynchronous (without a clock signal): where two devices exchange data serially,
one bit at a time, using two data lines + a common ground.

Both parties must agree on how fast data will be sent.
The common configuration is: 1 start bit, 8 bits of data, 1 stop bit, baud rate of 115200 bps.

Today's computers don't have serial ports, but our SoC has a USB-to-serial converter:
it exposes a serial interface to the microcontroller, and a USB interface to the computer, which will see the microcontroller as a virtual serial device.

The computer will see it as a TTY device: `/dev/ttyUSB0` or `/dev/ttyACM0`.
You can send out data by simply writing to this file:

```console
$ echo 'Hello, world!' > /dev/ttyACM0
```

Here's how to open a terminal:

```console
$ screen -U /dev/ttyUSB0 115200
C-a \

$ minicom --device /dev/ttyUSB0 -b 115200
C-a x

$ picocom /dev/ttyUSB0 --b 115200
C-a C-x
```

## UART

The microcontroller has a peripheral called UART: Universal Async Receiver/Transmitter.
It can be configured to work with several communication protocols: e.g. serial.
We'll use it to talk to your computer.

