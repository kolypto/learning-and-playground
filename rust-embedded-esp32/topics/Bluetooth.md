# Bluetooth

Bluetooth categories:

* Bluetooth Classic: for devices that require continuous data transfer. Higher data rates.
  Previously called just "Bluetooth", now "Classic" — to distinguish it from BLE.
* Bluetooth Low Energy (BLE): designed for low power consumption.
  Takes less time to set up a connection. Ideal for IoT.

ESP32 supports *Dual-Mode Bluetooth*: i.e. both modes are available simultaneously.

## BLE Stack

* RADIO
* Controller layers:
  * PHY: Physical Layer
  * LL: Link Layer
  * Isochronous layer
* **HCI**: Host Controller Interface
* Host:
  * **L2CAP**: Logical Link Control & Adaptation Protocol
  * **SMP**: Security Manager
  * **ATT**: Attribute Protocol
  * **GATT**: Generic Attribute Profile
    How devices exchange and structure data
  * **GAP**: Generic Access Profile:
    How devices connect and communicate
* APP: Your application

GATT — how devices exchange and structure data.
Defines how BLE devices exchange data. It organizes data in a hierarchy of *services*
and characteristics, allowing clients to read, write, and subscribe to updates from a BLE peripheral.

GAP — how devices connect and communicate.
It covers device roles (e.g. central, peripheral), connection parameters, security modes.

### GAP

GAP: Generic Access Profile — discover, connect, communicate with devices.

BLE communication ways:

1. Connected communication: two devices, a direct connection, duplex data exchange
2. Broadcast: a bluetooth beacon continuously sending updates

Device roles:

* Broadcaster: the beacon
* Observer: beacon receiver
* Central: connected communication. Initiator.
* Peripheral: connected communication. Advertises iteslf, accepts connection.

BLE peripheral discovery modes:

1. Non-Discoverable: cannot be discovered or connected to.
   This is the default mode when a connection is established, or when no advertising is active.
2. Limited-Discoverable: discoverable for a limited time to save power.
3. General-Discoverable: advertises indefinitely until a connection is established

Advertisement flags:

* Discoverable: limited or general?
* Bluetooth Classic: possible?
* BLE + Classic: possible at the same time?


### ATT and GATT

After the GAP layer helps BLE devices find each other and connect, ATT and GATT layers
define how data is structured and transmitted between devices.

In the GATT, there are two roles:

* Server: holds data as attributes.
  E.g. a peripheral device (like a sensor) acts as a server.
* Client: accesses the server's data

Roles may swap: e.g. the smartphone (client) reads fitness data from a tracker (server),
but then the smartphone (server) needs to send configuration to the tracker (client).

ATT: Attributes. Defines how data is stored as attributes.
Each attribute has:
* a unique handle
* type (16-bit identifier or 128-bit UUID)
* permissions (readable, writable)
* data: the actual value.
The client can read, write, or subscribe to data.

Attributes are like remotely accessible registers: a software abstraction of those.
They are stored in RAM that the BLE stack exposes over the air. And unlike registers,
they are structured.

GATT adds structure to the data: identifies how data is grouped and accessed.
Attributes are organized into:

* **Characteristic**: a single piece of data that a device can share.
* **Service**: a collection of related characteristics
* **Profiles**: a collection of related services

Example:

> Heart Rate Profile => Device Info Service (make, model, sn) ; Heart Rate Service (heart rate measurement; body sensor location)

Each service and characteristic should have a unique ID value.
The UUID could be either a *standard Bluetooth-SIG defined UUID* (16-bit)
or a custom UUID (128 bit).

See: [Assigned Numbers](https://www.bluetooth.com/specifications/assigned-numbers/) — the list of pre-defined UUIDs.

### BLE Address

BLE Address is like a MAC address: 48 bit hex.
Two main types of BLE addresses:

* Public Address: A public address is a permanent, worldwide-unique code given to a device by its manufacturer.
  It never changes and is registered with the IEEE. Only one device can have each address, and getting one requires paying a fee.
* Random Address: A random address is used more often because you don't need to register it with the IEEE, and you can set it up yourself.

  Random addresses can be further classified into:

  * Static: Stays the same until you restart the device
  * Private (dynamic): Changes over time to protect your privacy.
    Random addresses help to protect privacy by hiding the device's real identity.

# Advertisement

A BLE device advertises itself.
There are 3 binary options:

* Connectable/Unconnectable: Client can connect and establish a GATT session (read/write characteristics)?
  Unconnectable: Broadcast only, no connections allowed (beacon mode)
* Scannable/Nonscannable: Client can request more info with a scan request (gets scan response packet).
  Nonscannable: Advertising data only, no additional scan response
* Directed/Undirected: Advertisement targets a specific device (by address) - fast reconnection.
  Undirected: Broadcast to everyone.

Examples:

* Connectable + Scannable + Undirected:
  Normal peripheral device. Advertises to everyone, allows connections, provides extra info on scan.
* Non-connectable + Non-scannable:
   Beacon mode. Just broadcasts data (temperature sensor, iBeacon). No connections.
* Connectable + Directed:
  Fast reconnect to known device. "Hey, device X, I'm here, connect immediately."

Why it matters:

* Beacons: non-connectable saves power (no connection overhead)
* Scannable: extra 31 bytes for device name/info without connecting
* Directed: millisecond reconnects vs seconds for undirected





## Rust Crates

* [Bleps](https://github.com/bjoernQ/bleps): A toy-level BLE peripheral stack. Easy to use. No async. Only ESP32.
* [Trouble](https://github.com/embassy-rs/trouble): Hardware-agnostic. More feature-complete. Async. Portable across chips.

## Links

* [The Bluetooth® Low Energy Primer](https://www.bluetooth.com/bluetooth-resources/the-bluetooth-low-energy-primer/)
* [BLE Fundamentals](https://academy.nordicsemi.com/courses/bluetooth-low-energy-fundamentals/)
* [More Examples: embassy-rs/trouble/examples](https://github.com/embassy-rs/trouble/tree/main/examples/esp32)
