OLED Dino with an Accelerometer
===============================

This project was supposed to be a simple I2C OLED display test, but in the end it's the Chrome Dino game :D

1. Init I2C bus as a *shared bus*: i.e. multiple drivers can use it
2. Init the OLED display using a driver
3. Print out some text. Demo.
4. Starts a Dino game with sprites and animation
5. Spawns a thread that detects you jumping with the accelerometer

Wiring for ESP32-C3:

1. OLED display:
   * GND -> GND
   * VCC -> 3.3V
   * SCL -> GPIO9
   * SDA -> GPIO8

2. ADXL345 accelerometer:
   * GND -> GND
   * VCC -> 3.3V
   * CS -> self VCC
   * SDO -> self GND
   * SDA -> GPIO8
   * SCL -> GPIO9
