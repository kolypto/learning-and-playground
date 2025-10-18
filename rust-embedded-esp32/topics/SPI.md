# SPI

SPI is de-factor standard for synchronous serial communication in embedded systems.
Originally invented by Motorola in ~1980s.
Allows interfacing with peripheral chips, LCD displays, ADC/DAC converters, flash, EEPROM, and other chips.

SPI follows a master–slave architecture, where a master device orchestrates communication
with one or more slave devices by driving the clock and chip select signals.

It uses 4 wires to support full duplex (FDX).
In contrast to 3-wire variants which are half-duples (HDX): one direction at a time.

SPI provides higher throughput than I²C or SMBus, but it requires more pins.

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

## Chip Select

How chip-select works: when the *chip select* pin is held in the inactive state,
the chip remains "deaf" and pays no heed to changes in the state of its other input pins:
it holds its outputs in the high impedance state (electrically disconnected),
so other chips can drive those signals.
When the chip select pin is held in the active state, the chip or device assumes that
any input changes it "hears" are meant for it and responds as if it is the only chip on the bus.

This is called "tristate output": a *digital buffer* that has three stable states:
a high voltage output state (logical 1), a low output state (logical 0),
and a high-impedance (Hi-Z) state. In the Hi-Z state, the output of the buffer is effectively
disconnected from the subsequent circuit.
(A *buffer* is a circuit that "copies" the input without drawing current.)

To begin communication, the SPI master first selects a slave device by pulling its S̅S̅ low.
(The bar above S̅S̅ indicates it is an active low signal, so a low voltage means "selected",
while a high voltage means "not selected")

Caveat: All S̅S̅ signals should start HIGH before sending initialization messages to any slave.
Either configure your S̅S̅ GPIOs to be initially HIGH, or add a pull-up resistor on each S̅S̅,
to ensure that all S̅S̅ signals are initially high.

## Data Transfer

Each device internally uses a [shift register](https://en.wikipedia.org/wiki/Shift_register) for serial communication,
which together forms an inter-chip circular buffer. (A *shift register* is a cascade of flip-flops:
latches with two stable states that can store a bit of information. The cascade shares the clock signal,
which causes the data stored in the system to shift from one location to the next.
By connecting the last flip-flop back to the first, the data can cycle in the register.)

Data is usually shifted out with the most-significant bit (MSB) first.

## Clock

During each SPI clock cycle, full-duplex transmission of a single bit occurs.
The master sends a bit on the MOSI line while the slave sends a bit on the MISO line,
and then each reads their corresponding incoming bit. This sequence is maintained even when
only one-directional data transfer is intended.

The Master must also configure the clock polarity and phase with respect to the data.
Motorola called this CPOL and CPHA (Clock POLarity and Clock PHAse).

Two options for CPOL:

* `CPOL=0`: a clock which idles at the logical low voltage.
* `CPOL=1`: a clock which idles at the logical high voltage.

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
* Expander configurations: use SPI-controlled addressing units to add chip selects
  using demultiplexers.


## Variations

* Extra timing: Devices often require extra clock idle time: before the first clock, or after the last one, or between a command and its response.
* Dual SPI, Quad SPI: use additional data lines to transfer more bits at a time
* DDR: Transfer 2 bits per clock cycle
