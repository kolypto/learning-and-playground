# Electronic

## LED and Resistors

LEDs: longer leg is usually the anode: "+".

Use the Ohm's law to choose a resistor:

> R = (Vs - Vf) / If.

"LED forward voltage" (Vf) is the voltage drop across an LED:
usually it is 1.8V..2.0V.

The remaning voltage must be dissipated on a resistor.

For most LEDs, the safe current is 20mA. Use 10mA if you want to be on the safe side.

For the GPIO, the safe current is 20mA, absolute max = 40mA.

You can find the Vf out using the "diode testing mode `-|◀-` on a multimeter —
or connect it to a power source (through a resistor!) and measure the voltage across it.

So, if your `Vs`=3.3V and your `Vf`=2.0V, and the `If`=10mA,
you'll need a `130Ω` resistor.

I've also seen `330Ω` and even `1kΩ` resistors: a safe choice for an unknown LED.


## Voltage Drop

How to drop some volts?

1. Voltage divider: you'll waste a lot of current
2. Use a diode: every diode drops 0.7V. It dissipates the energy as heat.
3. Stabilitron: for small loads
4. LDO-module: Low-Dropout Regulator. MP2315, LM3940, ... . They will still heat up.
5. Switching regulator: doesn't heat up. Can increase or decrease voltage.
6. Level shifter, translator: converts digital signals from one logic standard to another.
   It's low power: i.e. it isn't meant to provide power
7. Linear regulator: can only be used to produce a lower voltage from a higher one

Chips for voltage regulation / level shifting:

* VHCT125A.  It's a 4-bus buffer, but can act as a level converter.
* AMS1117. A low-dropout voltage regulator: 5V to 3.3V linear regulator

### Voltage Divider

The HT-SR04 Ultrasonic Sensor is powered by 5V, which is fine.
But its output pin (echo) produces 5V as well, which ESP32 can't handle:
ESP32 is only around 3.6V tolerant on its GPIO pins.

Therefore we connect the "echo" output to GPIO *via voltage divider*:

<img src="https://esp32.implrust.com/ultrasonic/images/ESP32-HC-SR04-circuit.png">

```
                     ┌─────────── GPIO
 5V output ───[ 1k ]─┴──[ 2k ]─── GND
```
