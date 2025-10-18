# Components

## Buzzer

There are two kinds of buzzers:

1. Active buzzer: have an internal oscillator => produce fixed frequency sound.
2. Passive buzzer: just a speaker.

How to identify:

* Active buzzer: apply constant voltage => produces sound
* Passive buzzer: apply voltage — will just click, but not buzz.
* Active buzzers may have a third pin: that's transistor base
* My buzzer was 3.3V

## LDR: Light-Dependent Resistor

Photoresistor: changes its resistance based on the amount of light falling on it.
The brighter the light, the lower the resistance, and the dimmer the light, the higher the resistance.

Mnemonic: Dracula. In sunlight he gets weaker :D But in the dark, he gets stronger.

You would typically connect it to one of the ADC pins (analog input) through a voltage divider.
Take a 10K resistor or larger, connect the GPIO pin in-between:

```
                        ┌──────────── GPIO
 3.3V output ───[ 10k ]─┴──[ LDR ]─── GND
```

The higher its resistance is, the higher the voltage: it produces a voltage drop
between the GPIO (central point) and the GND. The 3.3V gets dropped in proportion
to the ration between the two resistors.

## Thermistor

Changes its resistance based on temperature.
All resistors change with temperature, but thermistors do this in a predictable manner.

Thermistors are categorized into two types:

* NTC (Negative Temperature Coefficient): Resistance decreases as temperature rises.
* PTC (Positive Temperature Coefficient): Resistance increases as temperature rises.

NTSs are primarily used for temperature sensing and inrush current limiting.
PTCs primarily protect against overcurrent and overtemperature conditions as resettable fuses
and are commonly used in air conditioners, medical devices, battery chargers, and welding equipment.

Connection: voltage divider.
Read voltage with ADC.

Example: NTC 103 Thermistor: 10K OHM. Use another 10K resistor.

### Thermistor Equation

To convert a thermistor's resistance to temperature, use:

* the Steinhart-Hart equation: more accurate.
  You'll need to know `A`, `B`, `C` constants from the datasheet
  that are specific to the thermistor's material
* the B equation: simpler but less precise.
  You'll need to know the `B`-value, the `T0` and `R0` from the datasheet.

The B-equation:

  $1/T = 1/T_0 + (1/B) * ln(R/R_0)$

* $T$: temperature in K (Kelvin)
* $T_0$: thermistor's reference temperature; usually 25℃
* $R$: thermistor's measured resistance
* $R_0$: thermistor's reference resistance at $T_0$; often $10 K\Omega$
* $B$ is the *B-value* of the thermistor: constant based on its material. Typically $3950$

You can determine the constants by making a series of experiments: room temperature, ice water, boiling water.
See: [Calculating Steinhart-Hart Coefficients](https://esp32.implrust.com/thermistor/steinhart.html)


### Math

The voltage divider: either R1 or R2 can be the thermistor. No big deal.

```
                        ┌──────────────── GPIO4
  V_DD ──────[ R_1 ]────┴───[ R_2 ]────── GND
```

The voltage divider formula:

  $V_{out} = V_{DD} * R_2 / (R_1 + R_2)$

To solve for R:

  $R_1 = R_2 * (V_{DD}/V_{out} - 1)$

  $R_2 = R_1 / (V_{DD}/V_{out} - 1)$

The ADC, however, measures voltage according to its reference value, $V_{ref} = 1100 mV$:

  $V_{measured} = V_{ref} * {adc\_value} / {ADC\_MAX}$

with attenuation, e.g. $dB=11$:

  $V_{measured} = V_{ref} * 10^{dB/20} * {adc\_value} / {ADC\_MAX}$
