# UART

UART: Universal Asynchronous Receiver-Transmitter.

* Asynchronous: not synchronized by a clock signal. Synchronization information is embedded in the data itself:
  start and stop signals set before and after each payload transmission.
* Data format and transmission speeds are configurable
* Sends data bits one by one, LSB-first.
* Data is framed by start and stop bits so that precise timing is handled by the communication channel.
* Only uses two wires
* Has error checking: 1 bit checksum

Connection: Tx to Rx, Rx to Tx.

## Baud Rate

There's no clock signal, but there's *baud rate*: speed of transfer, bps (bits per second).

In UART, both the transmitting and receiving devices must agree on the same baud rate to ensure successful communication.
The rx/tx baud rate can only differ by about 10%: otherwise, errors arise.


## The Protocol

Packet:

```
 -----------------------------------------------------------------------
|  1 start bit  |  5-9 data bits  |  0-1 parity bits  |  1-2 stop bits  |
 -----------------------------------------------------------------------
                   \ Data frame /
```

The UART line is normally held at HIGH voltage when idle.

Start bit: pulls the line LOW for one clock cycle.
This signals the receiver to start receiving.

Data frame: the payload. 5-8 bits + parity bit, or 9 bits with no parity bit.
Order: LSB-first.

Parity bit: the checksum. It's set to `0` when the number of `1`s is even, `1` if odd.

Stop bit: signals the end of the data packet.
The sender pulls the line to HIGH for at least 2 bit durations.


## Hardware

It's not just a protocol: it's a physical circuit in a microcontroller
that converts parallel data (e.g. a whole byte) into serial form: bit by bit.

Essentially, the UART is an intermediary between parallel and serial interfaces.
It connects a *data bus* to a remote end using just two wires.
Input: 8 data linse + CLK + INT + R/W; Output: Rx, Tx

Some microcontrollers may have multiple UART peripherals.

More advanced UARTs may throw their received data into a buffer, where it can stay until the microcontroller comes to get it.

Software emulation: if a microcontroller doesn't have a UART, the serial interface
can be *bit-banged*: i.e. directly controlled by the processor.
Note that bit-banging is CPU-intensive and not usually as precise.


## Flow Control

Flow Control is a mechanism where the receiver can tell the sender that it's overwhelmed
and cannot keep up.

* RTS/CTS flow control, Hardware flow control: uses 2 extra wires to signal the transmitter to stop & resume.
  They are called RTS (Request to Send) and CTS (Clear to Send).
  These wires are cross-coupled: RTS to CTS, CTS to RTS.
  Each device will use its RTS to output if it is ready to accept new data and read
  CTS to see if it is allowed to send data to the other device.
* Legacy hardware flow control: it also uses two wires, but it is unidirectional, with master/slave relationship.
  Connection is straight coupling: RTS-RTS, CTS-CTS.
  When the master wants to transmit data to the slave it asserts the
  RTS line. The slave responds by asserting CTS. Transmission can then occur until the slave deasserts CTS, indicating that it needs a
  temporary halt in transmission. When the master has finished transmitting the entire message it will deassert RTS.
* Software flow control: does not use extra wires. Instead, transmission is started and stopped by
  sending special flow control characters: typically, ASCII codes XON and XOFF (0x11 and 0x13).

More info: [UART Flow Control](https://www.silabs.com/documents/public/application-notes/an0059.0-uart-flow-control.pdf)

## Links

* [CircuitBasics: Basics of UART Communication](https://www.circuitbasics.com/basics-uart-communication/)
* [SparkFun: Serial Communication](https://learn.sparkfun.com/tutorials/serial-communication/uarts)
* [UART Flow Control](https://www.silabs.com/documents/public/application-notes/an0059.0-uart-flow-control.pdf)