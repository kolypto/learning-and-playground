# ADC: Analog-to-Digital Converter

ADC converts analog signals into digital.

## ADC Resolution

*ADC resolution* is the number of bits it uses to represent a reading:
it is also the precision:

* 8 bits: values between 0..255
* 10 bits: 0..1023
* 12 bits: 0..4096

In relation to a reference voltage:

> resolution = Vref / (2^bits - 1)

This means that for a 12-bit ADC and a 3.3V input, the resolution is 0.8 mV:

> 3.3V / (2^12−1) = 3.3V / 4095 = 0.8 mV

ESP32-C3 has two 12-bit SAR ADCs, up to 6 channels.
NOTE: ADC2 is not calibrated; also, on some chip revisions, it is not operable due to a bug!
Use ADC1 instead. (See: [errata](https://espressif.com/sites/default/files/documentation/esp32-c3_errata_en.pdf))

NOTE: The ADC in the ESP32 is known to have non-linearity issues.

## Pins

Not all pins are available for analog signal processing!
See "IO MUX" chapter.

In ESP32-C3:
* pins 0..4 are available to ADC1
* pin 5 is available to ADC2 (uncalibrated!)


## Reference voltage

The ADC needs a reference voltage to compare with the input voltage. This reference voltage,
called *V_ref* (Voltage Reference), helps the ADC map input voltages to digital values.

The ESP32 uses a $V_{ref} = 1100mV$.
This means it can only map input voltages between 0nV and 1100mV.

However, due to manufacturing variations, the actual value may range
between $1000 mV$ and $1200 mV$ depending on the chip
(see: [Reference: ADC](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-reference/peripherals/adc/index.html)). Calibration needed!

The SAR resolution is 12 bits: $2^{12} - 1 = 4095$.
The output value:

  $V_{data} = V_{ref} * {adc} / 4095$

## Attenuation

But what happens when the input voltage is higher than 1.1V?
Use programmable *attenuation*: it reduces input voltages to fit into the range.
Attenuation is configurable in the code, in dB.

dB (decibels) express ratios logarithmically:

  ${dB} = 20 × log₁₀(V_{out} / V_{in})$

However, the [Reference: ADC](https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/api-reference/peripherals/adc/index.html)) gives this table:

* $db=0$, range = 0 mV ~ 750 mV
* $dB=2.5$, range = 0 mV ~ 1050 mV
* $dB=6$, range = 0 mV ~ 1300 mV
* $dB=11$, range = 0 mV ~ 2500 mV

And this is how you find the actual reading:

  $V_{measured} = V_{max} * {adc\_value}/{ADC\_MAX}$

Mind the ADC non-linearity at the extremes.
Near 0V and near max, the ADC readings become inaccurate and noisy.
Therefore keep your range in the middle.

TODO: read about ADC calibration!
<https://docs.espressif.com/projects/rust/esp-hal/latest/esp32c3/esp_hal/analog/adc/index.html>
Uncalibrated ADC can give 30% errors and more!


## Example: Read Voltage from GPIO

```rust
// ADC Input
// Attenuation: -11dB
let ldr_pin = peripherals.GPIO0;
let mut adc_config = adc::AdcConfig::new();
let mut pin = adc_config.enable_pin(ldr_pin, adc::Attenuation::_11dB);
let mut adc1 = adc::Adc::new(peripherals.ADC1, adc_config);

// Value: 0..4095
let value: u16 = match nb::block!(adc1.read_oneshot(&mut pin)) {
    Ok(result) => result,
    Err(err) => {
        log::error!("err={err:?}");
        continue;
    }
};
```

## Example: Read Voltage from