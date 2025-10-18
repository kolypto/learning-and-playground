# SD Card

## Bus Mode
SD Cards can work in SPI mode!

All SD card families initially use a 3.3 volt electrical interface.
On command, SDHC and SDXC cards can switch to 1.8 V operation.

Like most memory card formats, SD is covered by patents and trademarks. Royalties apply to the manufacture
and sale of SD cards and host adapters, with the exception of SDIO devices.
NOTE: SPI mode does not require a host license. SDcard mode does. Also check: MMC mode.

The full details of the SD Bus protocol are not publicly available and can only be accessed through the SD Association.

## Command Interface

SD cards and host devices initially communicate through a synchronous one-bit interface, where the host device
provides a clock signal that strobes single bits in and out of the SD card. The host device thereby sends 48-bit
commands and receives responses. The card can signal that a response will be delayed, but the host device can abort the dialogue.

Through issuing various commands, the host device can:
* Determine the type, memory capacity and capabilities of the SD card
* Command the card to use a different voltage, different clock speed, or advanced electrical interface
* Prepare the card to receive a block to write to the flash memory, or read and reply with the contents of a specified block.

The command interface is an extension of the MultiMediaCard (MMC) interface. SD cards dropped support for some of the commands
in the MMC protocol, but added commands related to copy protection. By using only commands supported by both standards
until determining the type of card inserted, a host device can accommodate both SD and MMC cards.

At power-up or card insertion, the voltage on pin 1 selects either the Serial Peripheral Interface (SPI) bus or the SD bus.
The SD bus starts in one-bit mode, but the host device may issue a command to switch to the four-bit mode, if the SD card supports it.

## Clock Speed
After determining that the SD card supports it, the host device can also command the SD card to switch
to a higher transfer speed. Until determining the card's capabilities, the host device should not use
a clock speed faster than 400 kHz.

SD cards other than SDIO (see below) have a "Default Speed" clock rate of 25 MHz.

## Power Consumption

The power consumption of SD cards varies by its speed mode.

During transfer it may be in the range of 66–330 mW (20–100 mA at a supply voltage of 3.3 V).
Standby current is much lower, less than 0.2 mA for one 2006 microSD card.

Modern UHS-II cards can consume up to 2.88 W, if the host device supports bus speed mode SDR104 or UHS-II.
Minimum power consumption in the case of a UHS-II host is 720 mW.

## Security
The host device can command the SD card to become read-only: reversible or permanent.
Most full-size SD cards have a mechanical write-protect switch.

A host device can lock an SD card using a password of up to 16 bytes.
A locked card rejects commands to read and write data.

## Pins

1. -
2. CS: Chip Select
3. DI: MOSI
4. VDD: +3.3V
5. SCK
6. GND
7. DO: MISO
8. -


# SD Card Markings

## Capacity standards

* SDSC: max 2 Gb, FAT12, FAT16
* SDHC: max 32 Gb, FAT32
* SDXC: max 2 Tb, exFAT (required by the SDXC standard, but can be reformatted)
* SDUC: max 128 Tb, exFAT

SDHC cards are physically identical to SD (SDSC) cards.

### Filesystem

Note on filesystem: reformatting an SD card may make the card slower, or shorten its lifespan!
Some cards use *wear-leveling algorithms* that are designed for the access patterns typical of FAT12, FAT16 or FAT32.
In addition, the preformatted file system may use a cluster size that matches the erase region of the physical memory on the card;
reformatting may change the cluster size and make writes less efficient.

The SD Association provides freely downloadable SD Formatter software to overcome these problems for Windows and Mac OS X.

## Bus Marks
Most relevant for handling large files.

* Default: 12.5 Mb/s — the original SD bus interface
* High-Speed: 25 Mb/s
* UHS-I, UHS-II, UHS-III: 50 Mb/s, 150 Mb/s, 312 Mb/s
* SD Express: 985 MB/s

UHS cards and devices use specialized electrical signaling and hardware interfaces.

* UHS-I cards operate at 1.8 V instead of the standard 3.3 V and use a four-bit transfer mode.
* UHS-II and UHS-III introduce a second row of interface pins to add a second lane of data transfer
  and use low-voltage differential signaling (LVDS) at 0.4 V to increase speed and reduce power consumption and electromagnetic interference (EMI).

SD Express incorporates a single PCI Express 3.0 (PCIe) lane and supports the NVM Express (NVMe) storage protocol.
They also support DMA (direct memory access).

## Speed Ratings

Speed classes overlap:

* C: Original speed class. C2, C4, C6, C10. Speeds: 2-10 Mb/s (minimum sustained write speed)
* U: UHS speed class. U1 = 10 Mb/s, U3 = 30 Mb/s
* V: Video speed class. V6, V10, V30, V60, and V90 = 6 Mb/s, ..., 90 Mb/s
* E: SD Express Speed Class. E150, E300, E450, and E600 — minimum sustained write speed
