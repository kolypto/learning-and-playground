package main

import (
	"fmt"
	// The machine package is the TinyGo hardware abstraction layer.
	// The interfaces that it exposes are the same regardless of the implementation of the target hardware.
	// The "device" package is a set of generated Go types that wrap low-level registers for a particular chip, but you normally don't use it.
	"machine"
	"time"
)

// Flash me:
// $ tinygo flash -target=esp32-mini32 -port=/dev/ttyUSB0 main.go

func main() {
	// The output: where to emit light?
    // led := machine.GPIO32
    led := machine.LED

    // Blinking LED
    if !true {
        // Configures the GPIO PIN.
        // GPIO PINs need initialization before they can be used.
        led.Configure(machine.PinConfig{Mode: machine.PinOutput})

        for {
            for i:=0; i<3; i++ {
                // Low. That's 0V
                // "Low" means it's connected to GND, "high" means it's connected to VCC (usually 3.3V or 5V)
                led.Low()
                time.Sleep(time.Millisecond * 200)
                led.High()
                time.Sleep(time.Millisecond * 200)
            }
            for i:= 0; i<3; i++ {
                led.Low()
                time.Sleep(time.Millisecond * 400)
                led.High()
                time.Sleep(time.Millisecond * 400)
            }

            // On some microcontrollers, the machine.Serial object is configured to send to the UART chip,
            // which is often wired to a USB-to-serial adapter chip on the dev board.
            // The adapter chip converts the serial bits into USB packets to the host computer.
            // On other microcontrollers, the USB controller is built directly into the microcontroller SoC.
            // The machine.Serial object on these microcontrollers is configured to send to the USB bus directly.
            //
            // In both cases, the microcontroller appears as a serial device on the host computer.
            //
            // Note that the "fmt" package increases the binary size. On small controllers, it may be worth avoiding it.
            // Use print() and println() instead.
            //      $ tinygo build -target=esp32-mini32 -size short                                                                                                                                                                                                                                                                                                                                  [01:07:56]
            //      code    data     bss |   flash     ram
            //      2752       0    4172 |    2752    4172

            //
            // Watch:
            // $ tinygo monitor
            // Flash and watch:
            // $ tinygo flash -target=esp32-mini32 -port=/dev/ttyUSB0 -monitor -port /dev/ttyUSB0
            // With other commands:
            // $ screen -U /dev/ttyUSB0 115200
            // $ minicom --device /dev/ttyUSB0
            // $ picocom -b 115200 /dev/ttyUSB0
            // $ python3 -m serial.tools.list_ports
            // $ python3 -m serial.tools.miniterm /dev/ttyUSB0 115200
            //
            fmt.Printf("SOS sent\n")
        }
    }


    // // Reading GPIO input
    if true {
        // Use the LED for output
        led.Configure(machine.PinConfig{Mode: machine.PinOutput})

        // Configure the pin as GPIO input
        inpin := machine.GPIO36
        // Normally, you connect it to VCC using a pull-up resistor with high resistance: ~10KOhm.
        // But through a button, it's connected to GND. Then, it's never floating.
        inpin.Configure(machine.PinConfig{
            // See also: `machine.PilInputPullup`: this adds an extra resistor from the pin to VCC, in software!
            Mode: machine.PinInput,
        })

        for {
            // Get the current state
            // It's `true` if it's connected to VCC, and `false` if connected to GND.
            // If it's not connected to anything, it's "floating": it will randomly change,
            // e.g. when you move your hand near the pin. This state is generally undetectable.
            value := inpin.Get()
            if value {
                for i:=0; i<10; i++ {
                    led.High()
                    time.Sleep(50 * time.Millisecond)
                    led.Low()
                    time.Sleep(50 * time.Millisecond)
                }
            } else {
                led.Low()
            }

            // Sleep
            time.Sleep(time.Millisecond * 100)
        }
    }



    // PWM.
    // NOTE: TinyGo doesn't support it *yet* on our ESP32 :(
    // if false {
    //     // The PWM peripheral.
    //     // It's a piece of hardware that's separate from the CPU but interfaces with it.
    //     pwm := machine.PWM0_PIN

    //     // Configure the peripheral wit the desired period.
    //     // NOTE: it is usually not exact and may differ between microcontrollers
    //     pwm.configure(machine.PWMConfig{
    //         Period: 1e9/500, // period: the time between rising edges
    //     })

    //     // The PWN peripheral to correspond to the LED pin.
    //     // Check your hardware documentation: it may say that "LED" PIN GPIO25 has "PWM4": this means it can be controlled by PWM4.
    //     // Here, we obtain the channel of the peripheral: the one that controls the `LED` PIN.
    //     ch, err := pwm.Channel(led)
    //     if err != nil {
    //         println(err.Error())
    //         return
    //     }

    //     for {
    //         // Blink the LED with variable intensity: fade-out effect.
    //         for i := 1; i<255; i++ {
    //             // Set the "duty cycle": ratio between the pulse duration and the period.
    //             // The threshold: it is expressed as a part of the `pwm.Top()` value: a percentage.
    //             // PWMs work by having a counter. When it's over the threshold, it goes "ON".
    //             pwm.Set(ch, pwm.Top()/uint32(i))
    //             time.Sleep(time.Millisecond * 5)
    //         }
    //     }
    // }



    if !true {
        // Read from the serial
        println("Reading from the serial port...")
        for {
            c, err := machine.Serial.ReadByte()
            if err == nil {
                if c < 32 {
                    // Convert nonprintable control characters to
                    // ^A, ^B, etc.
                    machine.Serial.WriteByte('^')
                    machine.Serial.WriteByte(c + '@')
                } else if c >= 127 {
                    // Anything equal or above ASCII 127, print ^?.
                    machine.Serial.WriteByte('^')
                    machine.Serial.WriteByte('?')
                } else {
                    // Echo the printable character back to the
                    // host computer.
                    machine.Serial.WriteByte(c)
                }
            }

            // This assumes that the input is coming from a keyboard
            // so checking 120 times per second is sufficient. But if
            // the data comes from another processor, the port can
            // theoretically receive as much as 11000 bytes/second
            // (115200 baud). This delay can be removed and the
            // Serial.Read() method can be used to retrieve
            // multiple bytes from the receive buffer for each
            // iteration.
            time.Sleep(time.Millisecond * 8)
        }
    }
}

