e-Ink
=====

e-Ink modules are generally black and white, but others support tri-color and even multi-color options.

SPI eInk Display
================

WaveShare e-Ink display with SPI interface.

## Refresh mode

e-Ink displays need to flicker between black and white several times to get rid of residual image.

Some variants support a Partial Refresh Mode: it skips the clearing phase, so they're faster —
but accumulate ghosting over time.
You typically do partial updates for minor changes, then periodic full refreshes to clean up.

eInk displays don't like to be updated too often.
Waveshare ones, for example, say not to full refresh any more frequently than 180 seconds (3 minutes) or go longer.
Why? Because eInk displays have a lifespan measured in refresh cycles (typically 100k+ full refreshes).

## Precautions

Link: <https://www.waveshare.com/wiki/2.13inch_e-Paper_HAT_Manual#Overview>

* After refreshing partially several times, you need to fully refresh EPD once.
* Note that the screen cannot be powered on for a long time. Set it to sleep mode or power it off.
* Refresh interval: >180s, at least once every 24h.
* Clear the screen before storing it.
* Working voltage: 3.3V (older board), 3.3V and 5V (newer board: v2.1 and newer).

## WaveShare Board Pinout

Power:
* VCC, GND: 3.3V or 5V

SPI:
* `DIN` (`MOSI`): data in.
* `CLK`: Clock
* `CS`: Chip select. When LOW, the display is active. Use any GPIO: it's not part of SPI.
* (There's no `MISO`: the display doesn't send anything.)

More:
* `DC` (Data/Command Select): Determines whether the data being sent is pixel data (HIGH) or a command (LOW).
* `RST`: Resets the display when pulled LOW and then HIGH. Essential for initialization.
* `BUSY`: Indicates whether the display is currently processing (HIGH) or ready to receive new commands (LOW).

### Connect to ESP32:

* `VCC`, `GND`: to 5V or to 3.3V
* `DIN` -> MOSI
* `CLK` -> SCK (SPI)
* `CS` -> GPIO output, active-LOW, initially HIGH
* `DC` -> GPIO output
* `RST` -> GPIO output, active-LOW, initially HIGH
* `BUSY` -> GPIO input, active-HIGH

Note: in my case it only worked when powered from 3.3V.
Maybe the logic shifter got confused and expected 5V logic when I powered it from 5V VCC.



eInk Panels
===========

You can also buy a eInk panel with a ribbon cable that will have 20-40 pins.
This is a panel and you'd need to get a compatible controller.


