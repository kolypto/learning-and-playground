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


## Voltage regulation

### Buck Converters

Buck Converters: lower voltage.

Buck converters uses a rapidly switching MOSFET to produce a signal with a duty cycle.
Output voltage = input voltage × duty cycle. (e.g., 80% duty = 4V from 5V).

Is very efficient (>90%) but produces noise.
They have a built-in reference voltage circuit: so output voltage doesn't really depend on the input voltage.

Modules:

* MP1584: modern, 3A
* LM2596: older, less efficient
* mini360 (lm2596-based)
* XL4005 (5A capable)
* XL4015 (8V+)


### Linear Regulators / LDO

LDO (Low-Dropout Regulator) work when the "voltage dropout" is small:
i.e. can operate even when the supply voltage is very close to the output voltage.

* LDO waste excessive power as heat, but do not produce any switching noise.
* LDO have smaller footprint. No inductor needed.
* LDOs respond to load changes in microseconds. Bucks are slower, but a capacitor would fix this.

Use cases:

* When the target voltage is very close to the input: buck converters typically need ~0.5-1V headroom to regulate.
* When the current is tiny
* When ultra-clean power required: audio, precision sensors, analog circuits, etc

Common pattern: Buck → LDO cascade.
Buck converter provides the major drop, LDO provides fine regulation.

Modules:

* ?

### Boost Converters

Boost Converters: raise voltage.

Boost converters use an inductor: in produces higher voltages to resist changes in current.

Modules:

* MT3608 - Dirt cheap, 2A, boosts up to 28V. Fixed 1.2MHz switching.
* XL6009 - 4A capable, adjustable up to 35V. More robust than MT3608. Good for higher power needs.
* TPS61xxx series (TI) - Professional grade. TPS61088 does 18W at 93% efficiency. TPS61322 is tiny, low-power option for battery projects. I2C control on some models.
* LT1618 (Analog Devices) - Bulletproof, wide input range, excellent documentation. Pricier but reliable.

### Buck-Boost Converters

Buck-boost converters can step voltage up or down, handling input voltages both above and below the target.
Essential when your input range crosses your output (e.g., Li-ion battery: 2.8V-4.2V → 3.3V).

Modules:

* TPS63070 (TI) - The workhorse. 2A, 96% efficiency, 2-16V in → adjustable out. Handles battery input gracefully. I2C power good signal. About $2-3.
* LTC3115 (Analog Devices) - 5A capable, super wide input (2.7-40V). Bulletproof for automotive or industrial. PowerPath switching between multiple sources.
* TPS63000/63001 - Cheaper siblings of TPS63070. 1.2A, good enough for most ESP32 projects. ~$1.
* TPS63900/TPS63901 - Low quiescent current (60µA), great for battery life. 500mA output. Perfect for always-on ESP32 deep sleep scenarios.
* MAX17222 (Maxim/Analog) - Tiny, 300mA, optimized for wearables. 1.8µA shutdown current.
* LM5118 - High voltage (3-75V in), industrial/automotive grade. Overkill for ESP32 but useful if you're messing with higher voltage systems.


### Level Shifters

Used for low-current signalling circuits, i.e.g logic:

* VHCT125A.  It's a 4-bus buffer, but can act as a level converter.
* AMS1117. A low-dropout voltage regulator: 5V to 3.3V linear regulator

### Other Ways to Drop some Volts

How to drop some volts?

1. Voltage divider: you'll waste a lot of current
2. Use a diode: every diode drops 0.7V. It dissipates the energy as heat.
3. Stabilitron: for small loads

#### Voltage Divider

The HT-SR04 Ultrasonic Sensor is powered by 5V, which is fine.
But its output pin (echo) produces 5V as well, which ESP32 can't handle:
ESP32 is only around 3.6V tolerant on its GPIO pins.

Therefore we connect the "echo" output to GPIO *via voltage divider*:

<img src="https://esp32.implrust.com/ultrasonic/images/ESP32-HC-SR04-circuit.png">

> 5V output --- 1K --- 2K to GND --- GPIO

```
                     ┌─────────── GPIO
 5V output ───[ 1k ]─┴──[ 2k ]─── GND
```

### Other Ways to Raise some Volts

* Charge Pumps (Switched Capacitor Converters)
* Flyback Converters (Transformer-Based)
* Voltage Multipliers (Cascaded diode-capacitor stages)
* Transformer + Rectifier (Inverter Approach)
* Piezoelectric Transformers

