# I2C: Inter-Integrated Circuit

I2C is:

* Synchronous: clocked, i.e. changes in the state of memory (flip-flops) are synchronized by a clock signal
* multi-master/multi-slave
* single-ended: one wire for reference voltage (GND) and another one for varying voltage; in contrast to differential signalling
* serial: one bit at a time; in contrast to parallel communication, where several bits are sent as a whole

Usage: attaching low-speed peripherals in short-distance, intra-board communication.

I2C bus was invented in 1980 by Philips Semiconductors (now NXP Semiconductors).

Features:

* Can control a network of device chips with just two GPIO pins

Variations:

* SMBus (commonly found on motherboards) is a stricter subset of I2C
* PMBus: power management bus
* Modern I2C systems incorporate some policies and rules from SMBus, sometimes supporting both

## Speed

Speeds:

* Standard mode: 100 kbits/s
* Fast mode: 400 kbits/s. Is highly compatible.
* Low-speed mode: 10 kbit/s
* Arbitrarily low clock frequencies are also allowed

Later revisions of I2C:

* Can host more nodes
* Can run at faster speeds:

  * Fast mode plus: 1 Mbit/s
  * High-speed mode: 3.4 Mbit/s
  * Ultra-fast mode: 5 MBit/s

The actual transfer rate is lower: because protocol overhead includes a target address, a register address, and per-byte ACK/NACK bits.

## Addressing

The number of nodes is limited by the address space, and also by the total bus capacitance of 400 pF.
This restricts practical communication distance to a few meters.
However, buffers can be used to isolate capacitance on one segment from another.

The bus has two roles:

* Master (controller): generates the clock and initiates communication
* Slace (target): receives the clock and responds when addressed by the controller

Address space: 7 bits, with a rarely used 10-bit extension.
This gives 112 addresses.

7-bit vs 8-bit confusion: Datasheets show addresses as 7-bit (`0x48`) or 8-bit with R/W bit included (`0x90`/`0x91`).
Rust I2C libraries want 7-bit. If you see a datasheet with `0x90`, shift right by 1:
`0x90 >> 1 = 0x48`.

Many devices have a fixed address defined by the manufacturer,
but others allow configuring the lower bits of the address using pins or jumpers.
This allows using multiple copies of the same chip on the same bus.

Reserved addresses:

* `0x00`: "General Call" address. For broadcasting.
  Example: a bus-wide reset, or driving multiple identical devies.
  Legacy feature, rarely used in modern designs.
* `0000 000 1`: Start byte
* `1111 1XX 1`: Device ID
* `1111 0XX X`: extension for 10-bit addressing
* Also: CBUS address, reserved, HS-mode controller code

## Modes of Operation

Modes of operation of a device:

* Master transmit
* Master receive
* Slave transmit
* Slave receive

Multiple masters can use the same bus by properly using
START and STOP messages (transactions): i.e. when one master
is done, another one acan use the bus.

## Pull-Up Resistors

Unlike UART or SPI connections, the I2C bus drivers are "open drain", meaning that they can pull the corresponding signal line low, but cannot drive it high.
Therefore, both SDA and SCL lines require pull-up resistors.

The value of pull-up resistors must be chosen to balance the rise time, power consumption, and signal integrity.

Start with 4.7K resistor ; adjust down if necessary (if the rise time is too slow).
For long buses / systems with lots of devices, smaller resistors are better.
Generally, 10K resistors work fine.

Note that many devices have pull-up resistors built into them.
If you have multiple decices on the same bus, you may need to remove some of those resistors.

### Rise Time and Capacitance
Rise time: the time it takes for a signal to transition from a low voltage level to a high voltage level.
The bus capacitance, or its ability to store a charge, affects rise time because it determines how quickly a circuit can charge.

Lower resistance values (Ex: 1 kΩ to 4.7 kΩ) allow more current to flow through the pull-up resistors. This increased current charges the capacitance of the bus lines more quickly, resulting in faster rise times which is vital in higher speed communications. While lower resistance values improve rise times, they can also lead to an excessive flow of current, causing unnecessary power consumption and potential damage to the components.

Higher resistance values (Ex: 10 kΩ) reduce the current flowing through the pull-up resistors, resulting in a slower charging of the bus line capacitance. This results in longer rise times, which can cause delays in signal transitions and potentially lead to bus errors or unreliable communication.

### Pull-Up Resistor Calculation

The spec suggests: calculate min resistance, max resistance, and choose one in-between.

Max resistance: based on bus capacitance and rise time. Measure the capacitance or estimate/simulate it.

Find the rise time on the device's datasheet. This is the time it takes to rise from V_IL(MAX)(0.3*VDD) to V_IH(MIN)(0.7*VDD)

Now:

> Rp(max) =  Trise/0.8473×Cbus, where 0.8473 is ln(0.7)-ln(0.3).

The min value is based on the supply voltage:
it must meet the minimum sink current requirements:

* 3 mA for Standard mode (100 kHz) or Fast mode (400 kHz)
* 20 mA for Fast mode Plus (1 MHz).

Now:

> Rp(min)= V_DD-V_OL(max)/I_OL

The trade-off between Rp(min) and Rp(max) is a tradeoff between speed and power:

* lower resistance values reduce power consumption and improve speed but increase current draw
* lower resistance improves the edges of signal transitions: they become more sharp
* higher values saves power, but may slow down signal transitions and lead to communication errors
* higher frequencies require lower resistance pull-ups: a lower resistance will charge/discharge the cable's capacitance faster.

Another formula [from the ATmega168 datasheet](https://electronics.stackexchange.com/a/1852/626574):

```
Freq < 100 kHz => Rmin = (Vcc - 0.4V) / 3ma , Rmax = 1000ns/Cbus
Freq > 100 kHz => Rmin = (Vcc - 0.4V) / 3ma , Rmax = 300ns/Cbus
```

The `Cbus` can be approximated: 10pF per pin × number of devices on the bus.


#### Save Power

Reality: I2C's open-drain design inherently wastes power.

Here's how to save power on the I2C bus if your device is battery-powered:

1. Use higher pull-ups. Works if your bus is short and slow (100 kHz).
2. Use lower bus voltage: 3.3V instead of 5V saves 33% power at same resistance.
3. Use internal pull-ups: many MCUs have configurable internal pull-ups (20-50KΩ).
   Weaker but draws less. Only works for very short buses.
4. Use SPI instead: SPI doesn't need pull-ups (push-pull outputs). More pins but lower idle power.
5. Disable pull-ups when idle: use GPIO+transistor to control pull-up power:

> MCU GPIO → transistor → pull-up resistors → I2C lines


## Implementation

I2C uses only two signals:

* SDA (or SDI): serial data line.
* SCL (or SCK): serial clock line. Generated by the master device.
* GND: common ground, of course

Typical voltages used are +5 V or +3.3 V.
Because devices don't actually drive the signals high, different voltages can co-exist on the same bus!
The trick is to connect the pull-up resistors to the lower of the two voltages.
But to be on the safe side, use logic converter or I2C level shifter.

A device pulls the SDA line low to transmit a bit. The line is high when idle, thanks to the pull-up resistor.
Communication is synchronized by the SCL line, with each bit transferred during each clock pulse.

A logic "0" is output by pulling the line to ground, and a logic "1" is output by letting the line float (output high impedance) so that the pull-up resistor pulls it high.
A line is never actively driven high. This wiring allows multiple nodes to connect to the bus without short circuits from signal contention:
i.e. multiple nodes may be driving the lines simultaneously.
If any node is driving the line low, it will be low.

NOTE: Because I2C is a shared bus, there is the potential for any device to have a fault and hang the entire bus.





### Clock Stretching

Slave devices can "stretch" the clock by holding the SCL line low to slow down communication if they need more processing time.
So, when someone pulls SCL low, it stretches the clock.
The controller will then have to wait until it goes high again before sending the next bit.

After the controller observes the SCL line going high,
it must wait an additional minimal time (4 μs for standard 100 kbit/s I2C) before pulling the clock low again.

### Arbitration
When someone pulls SDA low, this is called *arbitration*:
two controllers may start transmission at about the same time.

In contrast to Ethernet, whcih uses random back-off delays, I2C has a deterministic
arbitration policy to ensure one transmitter at a time.
Each transmitter checks the level of SDA and compares it with the level it expects.
If they do not match, that transmitter has lost arbitration and drops out.

The first node to notice such a difference is the one that loses arbitration.
In the meanwhile, another node sees no disturbance in the message and proceeds.

As with clock stretching, not all devices support arbitration.
Those that do, generally label themselves as supporting "multi-controller" communication.

### Sharing SCL
It is possible to have multiple I2C bses share the same SCL line.
The packets will be sent at the same time.

## Communication

In addition to 0 and 1 data bits, I2C allows special START and STOP signals that are distinct from the data bits.
They act as message delimiters.

* START: SDA goes high-to-low, with SCL high.
* STOP: SDA goes low-to-high, with SCL high.
* Data: all other transitions of SDA take place with SCL low.
* It is illegal, but harmless, to do multiple SDA transitions while SCL is high.

Sequence:

1. The master is initially in Master-Transmit mode.
2. It sends START + 7 bits of address of the target + 0 (write) or 1 (read)
3. The target responds with an ACK bit (active low).
4. Master continues in transmit or receive; target continues in the complementary mode: receive or transmit
5. After every 8 data bits in one direction, an "acknowledge" bit is transmitted in the other direction.
6. The controller terminates a message with a STOP condition; or it may send another START

Bit order: MSb (most significant bit first).

Pure I2C supports arbitrary message structures:
i.e. messages are product-specific.

SMBus is restricted to 9 specific commands — like "read word N" and "write word N".

### Combined Transaction

In addition to *single message* mode (read or write),
I2C defines a *combined transaction*: master issues multiple reads/writes to multiple targets.

* Each read/write begins with a START + target address.
* These *repeated START bits* are not preceded by STOP conditions:
  this is how targets know that the next message is part of the same transaction
* The terminating STOP indicates when those grouped actions should take effect

A combined transaction allows you to apply an operation atomically:
e.g. you can configure multiple paramenters on a power chip
and make sure the combined effect takes place.

Example: single transaction:

```
START → Address+Write → Data → STOP
START → Address+Read → Data → STOP
```

The problem is that another master may jump in between these two.

A combined transaction:

```
START → Address+Write → Data → RESTART → Address+Read → Data → STOP
```

When to use:

* Register read: write register address — then read its value without releasing the bus
* Multi-register operations:
Set multiple registers that must change together

When there's only one master on the bus, it doesn't matter.
But many I2C devices (example: EEPROM) require the *repeated START* pattern for register reads!
Example: An accelerometer expects you to send register address then immediately read without a STOP. The STOP would reset its read pointer.


## ESP32

The ESP32-C3 includes one I2C bus.
It supports up to 800 kbit/s, 7-bit and 10-bit addressing mode.
Only GPIO 8 and 9 can be used for I2C.

The ESP32-C6 has I2C controllers: one in the main system and one in the low-power system


## Rust

In Rust, `embedded-hal` (and `embedded-hal-async`) defines a trait.
Microcontroller-specific HAL crates (like `esp-hal`, `stm32-hal`, or `nrf-hal`) implement this trait.

Example:

```rust
// I2C. Init in async mode.
let i2c_bus = i2c::master::I2c::new(
    peripherals.I2C0,
    i2c::master::Config::default().with_frequency(Rate::from_khz(400)),
).expect("Init I2C bus").with_sda(peripherals.GPIO8).with_scl(peripherals.GPIO9).into_async();
```

The `embedded-hal-bus` crate provides wrapper types like `AtomicDevice`, `CriticalSectionDevice`, and `RefCellDevice`
that allow multiple drivers to safely share access to the same I2C bus.
It basically wraps the `I2C` instance so that you can
share it across multiple drivers.



## Links

* [SparkFun: I2C](https://learn.sparkfun.com/tutorials/i2c/all)
* [TI: A Basic Guide to I2C](https://www.ti.com/lit/pdf/sbaa565)
* [CircuitBasics: Basics of the I2C Communication Protocol](https://www.circuitbasics.com/basics-of-the-i2c-communication-protocol/)
* [`embedded-hal/i2c`: in-depth details](https://docs.rs/embedded-hal/latest/embedded_hal/i2c/index.html)
