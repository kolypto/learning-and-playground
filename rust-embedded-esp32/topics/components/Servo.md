# Servo

The SG90 offers up to 180° of rotation.

A servo motor usually has a DC motor, a control circuit, a gearbox, and a potentiometer.
The DC motor is linked to the output shaft through the gearbox, which moves the servo's horn.

To control the horn's position, we send a signal to the servo motor from the microcontroller (MCU)
at a frequency of 50Hz, with a pulse every 20 milliseconds. By changing how long the signal stays high
during each cycle (pulse width), we can control how far the horn rotates.

The servo motor holds its position until we change the pulse width.
If you send the wrong pulse width, it won't move at all.

Pins:

1. Orange = PWM
2. Red = VCC
3. Brown = GND

In the datasheet for SG90, the pulse pattern is:

* 1ms pulse => -90°
* 1.5ms pulse => 0°
* 2ms pulse => +90°

My actual motor, however, reacts differently:

* 0.625ms => -90°
* 2.575ms => +90°

At 50 Hz and max_duty=4096 (12 bits), this translates to:

* duty=128/4096 => -90°
* duty=527/4096 => +90°
