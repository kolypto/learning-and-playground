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
5. Switching regulator: doesn't heat up
