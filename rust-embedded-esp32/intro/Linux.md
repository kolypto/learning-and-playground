## Connect the Board

Connect board to your computer. Verify, a tiny red control LED lights up.
The device should also expose its UART serial port over USB.
It will show up in `lsusb`:

```console
$ lsusb | grep USB
Bus 006 Device 035: ID 303a:1001 Espressif USB JTAG/serial debug unit
```

Find the device by id:

```console
$ ls -l /dev/serial/by-id
lrwxrwxrwx 1 root root .... usb-Espressif_USB_JTAG_serial_debug_unit_60:55:F9:C0:27:18-if00 -> ../../ttyACM0
```

The device will either be `/dev/ttyACM0` or `/dev/ttyUSB0`.
This depends on on the USB-to-serial implementation:
* ACM (Abstract Control Model) driver is used by boards with native USB support or CDC-ACM USB-serial chips.
  The microcontroller itself handles USB, or uses chips that implement the CDC class.
  ACM devices generally support higher speeds and are considered "proper" USB devices rather than converters.
* ttyUSB: USB-serial converter driver. Used by boards with dedicated USB-to-UART bridge chips 
  like CP2102, CH340, FTDI, etc. These are separate chips that convert USB to plain serial.


