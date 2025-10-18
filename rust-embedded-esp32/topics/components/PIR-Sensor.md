# PIR Sensor (Motion Sensor)

PIR = Passive Infrared: because it does not emit any IR radiation; instead, it
only detects changes in infrared radiation from the environment. It senses heat emitted by people, animals, and other warm objects.

It has 3 pins:

1. Power: 5V
2. Output: HIGH when motion is detected, LOW otherwise. Conveniently, it's 3.3V
3. Ground

It has 2 potentiometeres:

1. Delay Time (Output Duration): determines how long the sensor’s output remains HIGH after detecting motion.
   A longer delay is useful for applications like automatic lighting, where you want the light to stay on for a while after movement is detected. Range: 5s .. 200s.
2. Sensitivity (Detection Range): controls how far the sensor can detect motion. Higher values => more distance, but also more false triggers. Range: 3m .. 7m.

Jumper setting:

1. Retriggering: the output stays HIGH as long as motion is detected. Use case: keep the lights on.
2. Non-Retriggering: the output stays HIGH once motion is detected but won't trigger again until the delay time is up. Use case: counting people.

