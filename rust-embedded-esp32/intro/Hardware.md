# ESP32 Family Chips

Old ESPs are all Xtensa:

* ESP32 (Wifi, BT): the original flagship. Dual core.
* ESP32-S: newer version. Higher performance. New hardware.

  Got OTG support: can connect to USB directly!
  Makes ch340 redundant: this is why some ESP32-S3 boards are physically smaller.
  Some boards still include a separate CH340 (or CP210x) chip: useful if the native USB code crashes or the chip is in a sleep mode.

    * S2: signle-core for power efficiency. Only Wi-Fi.
    * S3: dual-core, WiFi, BT. AI features: supports vector instructions.

Newer ESP chips are all RISC-V.

* ESP32-C (compact, cost-effective, communication): WiFi, Bluetooth, Zigbee/Thread. Not as performant as S3, though, and less RAM.

    * ESP32-C6: 2.4Ghz WiFi 6, BT 5, Zigbee, Thread. 160 Mhz. Works with external flash. It also includes a 20Mhz extra low power core.
    * ESP32-C61: 2.4Ghz WiFi 6, BT 5. Ultra low power.
    * ESP32-C5: 2.4 and 5 GHz Wi-Fi 6, BT 5, Zigbee, Thread. Ultra low power.
    * ESP32-C3: 2.4Ghz WiFi, BT 5. 160 Mhz. 384Kb ROM, 400Kb SRAM. Crypto peripherals. Allows connections to flash.
    * ESP32-C2: 2.4Ghz WiFi, BT 5. 120 Mhz. 576Kb ROM, 272Kb SRAM. Only 14 GPIOs. Cheapest and smallest.

* ESP32-H (hibernate): 	Focus on Thread/Zigbee, no Wi-Fi. Has Bluetooth. Low power consumption in sleep mode.
* ESP32-P (performance): high-performance chip: up 400 MHz. FPU, AI features. No wireless. Newest chips.


## Hardware Terminology

* System-on-a-Chip: the chip package itself. Includes the CPU and its peripherals, all in one package.
  SoCs are primarily intended for integration into custom hardware designs.
* Module: chip on a board with some resistors, crystal oscillator, antenna, flash memory, EMI shield, ... — a ready-to-use solution.
  Common examples: WROOM, WROVER series
* Development Boards (Devkit): have a USB interface, voltage regulator, pin breakouts, boot and reset buttons.


## ESP32 Chip Marking

### ESP32

**ESP32** D 0 WD R2 H Q6 V3:

* `D`/`U`: Dual Core; `S`: Single Core
* In-package flash: `0`=None, `2`=2Mb, `4`=4Mb
* Connection. `WD`: Wi-Fi b/g/n + Bluetooth/Bluetooth LE dual mode
* In-package PSRAM. `R2`: 2 MB PSRAM
* `H`: High temperature
* Package. `Q6`: QFN 6*6; *N/A*: QFN 5*5
* `V3`: Chip revision v3.0 or newer

### ESP32-C3

**ESP32-C3** F H/N 4 X

* `F`: Flash. Has flash.
* `H/N`: Flash temperature: `H`: High Temperature, `L`: Low Temperature
* `4`: Flash size, Mb
* `AZ`: Other identification code

### ESP32-C6

**ESP32-C6** F H/N 4

* `F`: Flash. Has flash.
* `H/N`: Flash temperature: `H`: High Temperature, `N`: Normal Temperature
* `4`: Flash size, Mb




## Flash

Note that the internal flash has limited number of write cycles.
ESP32 flash can handle 100.000 cycles at minimum.

## Peripherals

While the CPU is responsible for executing program logic, peripherals are hardware components
that extend its capabilities: they allow the MCU to interact with the outside world by handling
inputs and outputs, communication, timing, and more.
This allows the CPU to focus on critical tasks while peripherals handle specialized functions independently.

*Offloading* refers to the practice of delegating certain tasks to hardware peripherals
instead of doing them directly in software via the CPU.
This improves performance, reduces power consumption, and enables concurrent operations.

For example:

* A UART peripheral can send and receive data in the background using DMA (Direct Memory Access), while the CPU continues processing other logic.
* A Timer can be configured to generate precise delays or periodic interrupts without CPU intervention.
* A PWM controller can drive a motor continuously without the CPU constantly toggling pins.

Offloading is a key design strategy in embedded systems to make efficient use of limited processing power.

Common types of peripherals:

* *GPIO* (General Purpose Input/Output)	Digital pins that can be configured as inputs or outputs to interact with external hardware like buttons, LEDs, and sensors.
* *UART* (Universal Asynchronous Receiver/Transmitter)	Serial communication interface used for sending and receiving data between devices, often used for debugging and connecting modules like Bluetooth.
* *SPI* (Serial Peripheral Interface)	High-speed synchronous communication protocol used to connect microcontrollers to peripherals like SD cards, displays, and sensors using a master-slave architecture.
* *I2C* (Inter-Integrated Circuit)	Two-wire serial communication protocol used for connecting low-speed peripherals such as sensors and memory chips to a microcontroller.
* *ADC* (Analog-to-Digital Converter)	Converts analog signals from sensors or other sources into digital values that the microcontroller can process.
* *PWM* (Pulse Width Modulation)	Generates signals that can control power delivery, used commonly for LED dimming, motor speed control, and servo actuation.
* *Timer*	Used for generating delays, measuring time intervals, counting events, or triggering actions at specific times.
* *RTC* (Real-Time Clock)	Keeps track of current time and date even when the system is powered off, typically backed by a battery.


## Temperature Sensor

**ESP32-C3** has one internal temperature sensor.

It also has 2 SAR ADCs for measuring analog signals from six channels (6 pins)./
It samples voltages with 12-bit sampling resolution.

* SAR ADC1: can measure from GPIO 0..4
* SAR ADC2: can measure from GPIO 5 only

**ESP32-C6** has 1 SAR:

* SAR ADC: can measure from GPIO 0..6
