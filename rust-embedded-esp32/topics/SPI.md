# SPI

SPI: Serial Peripheral Interface.

* Master-slave architecture
* Synchronous: clocked
* Serial: transfers bits one by one rather than in parallel

SPI is de-factor standard for synchronous serial communication in embedded systems.
Originally invented by Motorola in ~1980s.
Allows interfacing with peripheral chips, LCD displays, ADC/DAC converters, flash, EEPROM, and other chips.

SPI follows a master–slave architecture, where a master device orchestrates communication
with one or more slave devices by driving the clock and chip select signals.

It uses 4 wires to support full duplex (FDX).
In contrast to 3-wire variants which are half-duples (HDX): one direction at a time.

* One unique benefit of SPI is the fact that data can be transferred without interruption:
  no start/stop bits and no taking turns.
* SPI provides higher throughput than I²C or SMBus, uses less power,
  but it requires more pins.
* No complicated slave addressing system like I2C
* Higher data transfer rate than I2C: almost twice as fast
* However, no acknowledgment that the data has been successfully received

As long as you have enough GPIO pins and don't need to connect a large number of devices,
SPI is usually the best tool for the job.


SPI is different from SSI: SSI employs
[differential signalling](https://en.wikipedia.org/wiki/Differential_signalling) (differential pair)
and provides only a single simplex communication channel.

## Logic Signals

Commonly, SPI has four logic signals:

* **S̅S̅**: Slave Select. Master sends it to select slave chip to communicate with.  Active-low signal.
* **SCLK**: Serial Clock. Clock signal from master.
* **MOSI**: Master Out Slave In. Serial data output from master.
  MOSI on a master outputs to MOSI on a slave.
* **MISO**: Master In Slave Out. Serial data output from slave.
  MISO on a slave outputs to MISO on a master.

So, every chip on the SPI bus shares 3 wires (SCLK, MOSI, MISO) with all other chips,
but the *S̅S̅* wire needs to be separate for every chip: one pin for every peripheral.

Alternative pin naming:

* **CS**, **SCL**, **SDI** (Serial Data Input) and **SDO** (Serial Data Output)
* **SPIQ**, **SPID**: MISO / MOSI nicknames
* **SDA** can reuse the name from I2C.
* **SDIO**: 3-wire SPI (Serial Data Input/Output)
* **FSPI**: fast API

## Chip Select

How chip-select works: when the *chip select* pin is held in the inactive state (HIGH),
the chip remains "deaf" and pays no heed to changes in the state of its other input pins:
it holds its outputs in the *high impedance state* (Hi-Z, electrically disconnected),
so other chips can drive those signals.

When the chip select pin is held in the active state (LOW), the chip or device assumes that
any input changes it "hears" are meant for it and responds as if it is the only chip on the bus.

To begin communication, the SPI master first selects a slave device by pulling its S̅S̅ LOW.
(The bar above S̅S̅ indicates it is an active LOW signal, so a LOW voltage means "selected",
while a HIGH voltage means "not selected")

Caveat: All S̅S̅ signals should start HIGH before sending initialization messages to any slave.
Either configure your S̅S̅ GPIOs to be initially HIGH, or add a pull-up resistor on each S̅S̅,
to ensure that all S̅S̅ signals are initially high.

The max number of slaves is theoretically unlimited,
but in practice is limited by the load capacitance of the system: high-capacitance wires
would fail to switch between voltage levels.

## Data Transfer

Each device internally uses a [shift register](https://en.wikipedia.org/wiki/Shift_register) for serial communication,
which together forms an inter-chip circular buffer. (A *shift register* is a cascade of flip-flops:
latches with two stable states that can store a bit of information. The cascade shares the clock signal,
which causes the data stored in the system to shift from one location to the next.
By connecting the last flip-flop back to the first, the data can cycle in the register.)

Data is usually shifted out with the most-significant bit (MSB) first.

## Clock

The speed of data transfer is determined by the frequency of the clock signal.

During each SPI clock cycle, full-duplex transmission of a single bit occurs.
The master sends a bit on the MOSI line while the slave sends a bit on the MISO line,
and then each reads their corresponding incoming bit. This sequence is maintained even when
only one-directional data transfer is intended.

The Master must also configure the *clock polarity and phase* with respect to the data.
Motorola called this CPOL and CPHA (Clock POLarity and Clock PHAse).

Two options for CPOL:

* `CPOL=0`: a clock where idle = logical LOW.
* `CPOL=1`: a clock where idle = logical HIGH.

Two options for CPHA:

* `CPHA=0`: The first data bit is output *immediately* when S̅S̅ activates.
  Data bits are sent when SCLK transitions *to* idle.
  Sampling occurs when SCLK transitions *from* idle.
* `CPHA=1`: The first data bit is output on SCLK's first clock edge after S̅S̅ activates.
  Data bits are sent when SCLK transitions *from* idle.
  Sampling occurs when SCLK transitions *to* idle.

The combinations of polarity and phases are referred to by these "SPI mode" numbers:
with CPOL as the high order bit and CPHA as the low order bit:

* SPI mode = 0: CPOL=0, CPHA=0 (send=falling SCLK, and when S̅S̅ activates; sample=rising SCLK)
* SPI mode = 1: CPOL=0, CPHA=1 (send=rising SCLK; sample=falling SCLK)
* SPI mode = 2: CPOL=1, CPHA=0 (send=rising SCLK, and when S̅S̅ activates; sample=falling SCLK)
* SPI mode = 3: CPOL=1, CPHA=1 (send=fallking SCLK; sample=rising SCLK)

However, because MOSI and MISO signals are usually stable,
devices may sample data at different points in that half cycle, despite the specification.

Mode 0 is the most common and works with most devices.

## Interrupt from Slave to Master

SPI slaves sometimes use an out-of-band signal (another wire) to send an interrupt signal to a master.

Examples: sensors, real-time clock chips, SDIO (SD card), audio jack insertions.

## Bus Topologies

SPI can communicate with multiple slaves.

* Multidrop configuration: each slave has its own S̅S̅.
  This is the way SPI is normally used.
* Daisy chain: first slave's output is connected to to the second slave's input, ...,
  until the final slave, whose output is connected back to the master's input.
  And they share S̅S̅.
  This effectively merges their shift registers.
  Devices must support this mode explicitly
* Expander configurations: use SPI-controlled addressing units to add chip selects
  using demultiplexers.


## Variations

* Extra timing: Devices often require extra clock idle time: before the first clock, or after the last one, or between a command and its response.
* Dual SPI, Quad SPI: use additional data lines to transfer more bits at a time
* DDR: Transfer 2 bits per clock cycle


## ESP32

ESP32-C3 has 3 SPI peripherals.
Only SPI2 is available for general use.

ESP32-C3 does not have on-chip memory and relies on an off-chip SPI flash.
SPI0/SPI1 pins for flash connection are not bonded for variants with 16 GPIOs.

By default `VDD_SPI` is the power supply pin for in-package and off-package flash.
It can be reconfigured as a GPIO pin.


## Rust

Two important traits for SPI are:

* `SpiBus`: Represents full control over the SPI bus, including the `SCK`, `MOSI`, and `MISO` lines.
  This must be implemented by the microcontroller's HAL crate: for example, `esp-hal` crate implements `SpiBus`.
* `SpiDevice`: Represents access to a single SPI device that may share the bus with others.
  It takes control of the chip select (CS) pin and ensures the device is properly selected before communication and released afterward.

The `embedded-hal-bus` crate provides ready-to-use wrappers that implement the SpiDevice trait for you.

If your project only uses one SPI device and doesn't need sharing, you can use the `ExclusiveDevice` struct.
But if your project has multiple SPI devices sharing the same bus, choose:

* `AtomicDevice`
* `CriticalSectionDevice`





## Links

* [CircuitBasics: Basics of API Communication Protocol](https://www.circuitbasics.com/basics-of-the-spi-communication-protocol)
* [SparkFun: SPI](https://learn.sparkfun.com/tutorials/serial-peripheral-interface-spi/all)
* [TI: SPI User Guide](https://www.ti.com/lit/ug/sprugp2a/sprugp2a.pdf?ts=1762512235969)

