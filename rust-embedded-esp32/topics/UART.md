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



## Client

Get a USB2TTL converter and send UART lines with `picocom`:

```console
$ picocom /dev/ttyUSB0 -b 115200 --omap 'crcrlf' --imap 'lfcrlf' 
```

Note that by default when you hit "Enter" it sends a "CR":
because back in the day, "Enter" meant "Carriarge Return", and there was a separate key for "Line Feed"
(see DEC VT05 from the 1970-s). Terminal emulators *emulate* exactly this behavior.
The `--omap` option converts your `\r` into `\r\n` when *outputting* it to the UART.


## Reading/Writing

You might want to split the peripheral to handle Rx and Tx separately:

```rust
// Init UART
// TODO: check DMA for faster I/O without CPU
let mut uart = uart::Uart::new(
    peripherals.UART1,
    uart::Config::default().with_baudrate(115_200),
    ).expect("UART init")
    .with_tx(peripherals.GPIO0)
    .with_rx(peripherals.GPIO1)
    .into_async();
let (rx, mut tx) = uart.split();
```

Writing:

```rust
// Send bytes
let data = b"Hello\n";
tx.write_async(data).await.expect("UART write");
```

Read bytes:

```rust
let mut buf = [0u8; 64];
loop {
    let n = uart.read_async(&mut buf).await.expect("UART read");
    defmt::info!("Uart read: {}", buf[..n]);
}
```

### Buffer

If you type manually, you will see that "read" reads one byte at a time.
This is because `read_async()` returns as many bytes as is available in the hardware Rx buffer,
which has 128 bytes in ESP32-C3. 

ESP32-C3 has:

* RX FIFO: 128 bytes hardware buffer
* TX FIFO: 128 bytes hardware buffer

Incoming bytes from the modem go straight into RX FIFO. When data arrives, UART hardware triggers an interrupt,
which in turn wakes up the task and `read_async()` reads whatever's in the FIFO (could be 1 byte, could be 50).

Interrupts: `esp-hal` configures these for you. 
RX interrupt fires when FIFO has data (or hits threshold). 
TX interrupt fires when FIFO has space.

If modem sends data faster than you read, you lose bytes, so the reader's got to be fast.
Or use `with_cts()` and `with_rcs()` for devices to agree on i/o readiness.

Also, you could use DMA that would let UART write directly to RAM without CPU intervention. 
But ESP32-C3 UART doesn't have DMA (ESP32-S3 does).

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
