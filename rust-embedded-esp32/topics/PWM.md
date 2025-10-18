# PWM: Pulse Width Modulation

Microcontrollers have binary logic: HIGH and LOW, with no in-between values.

PWM can generate an analog signal: a rectangular wave with varying *duty cycle* ­— without using the processor.
See: [Pulse-Width Modulation](https://en.wikipedia.org/wiki/Pulse-width_modulation).

Coupled with a capacitor, you can control the output voltage gradually.
If you microcontroller outputs 3.3V, then a PWM with duty cycle = 50% produces 1.65V on average.

Two parameters for a PWM:

1. Frequency
2. Duty cycle

Technically, it's implemented like this:

* A counter. It counts from 0 to a specific *maximum value* (stored in a register), then starts over.
* A *compare value* (stored in a register). When `counter < compare value`, the signal stays HIGH. When exceeds, the signal goes LOW.

*PWM Resolution*: how precisely can the duty cycle be controlled: is determined by the number of bits in the PWM register.
Example:

* 4-bit resolution => 16 duty cycle levels
* 8-bit resolution => 256 duty cycle levels
* 10-bit resolution => 1024 duty cycle levels.


## LED-PWM and MCPWM

Kinds:

* LED-PWM: simple dimming. Because when PWM switching happens very often, the eye cannot see it!
* Motor Control PWM: with more advanced features like fault detection and synchronization signals

ESP32-C3 has a LED-PWM controller with 6 channels, 4 independent timers, and 14 bits of resolution.
It also supports gradual increase/decrease of duty cycles (for dimming).

ESP-C6 has a LED-PWM with 20 bits.
It also has a Motor Control PWM (MCPWM): designed for driving digital motors and smart light.

## Architecture

LED PWM registers are clocked by `APB_CLK` (note that the `APB_CLK` signal to the LED PWM has to be enabled first
by setting the `SYSTEM_LEDC_CLK_EN` field in the register `SYSTEM_PERIP_CLK_EN0_REG`).

> `APB_CLK` (Advanced Peripheral Bus Clock) is the clock for a system's peripherals,
> while `CPU_CLK` is the clock for the central processing unit itself.
> `APB_CLK` is highly dependent on the `CPU_CLK` source.
> The main difference is that `APB_CLK` is slower: it basically is `CPU_CLK` divided:
> This division is necessary because peripherals don't need the same high speed. It's also more power efficient.

The 4 timers in ESP32-C3/C6 are identical in their features and operation: `Timer0`, `Timer1`, `Timer2`, `Timer3`.
Every timer maintains its own timebase counter.
The four timers can be independently configured, though: clock divider, counter overflow, etc.

Each PWM generator selects one of the timers and uses the timer’s counter value as a reference to generate
its PWM signal.

Timers can choose a clock signal:

* `APB_CLK` — a peripheral clock that is derived from these and other sources. Configurable frequency.
* `PLL_CLK`: (320 MHz or 480 MHz): internal PLL clock
* `XTAL_CLK` (40 MHz): external crystal clock
* `XTAL32K_CLK` (32 kHz): external crystal clock
* `RC_FAST_CLK` (17.5 MHz by default): internal, less stable, fast RC oscillator with adjustable frequency
* `RC_FAST_DIV_CLK`: internal fast RC oscillator derived from RC_FAST_CLK divided by 256
* `RC_SLOW_CLK` (136 kHz by default): internal low RC oscillator with adjustable frequency

A timer applies a *division factor* to the clock to slow it down.
The `clk_div=1..1023`.

## How to Get a Specific Frequency

ESP32-C3's LEDC (PWM) module uses a fixed 80 MHz APB clock as the source.
To hit a target frequency (e.g. 24 kHz), it divides the clock by a prescaler `D` and then
counts up to `2^duty_resolution_bits - 1`.

> resulting_pwm_freq = clock_freq / (clk_div * 2^duty_resolution_bits)

You pick `clk_div` first, then solve for the closest integer `duty_resolution_bits`
that gets you near 24 kHz. Higher bits => finer control, but forces larger `clk_div`:
possible frequence error if `clk_div` can't be exact.

So, you need to pick two parameters: `clk_div` and `duty_resolution_bits`.

In `esp-hal`, the `clk_div` is hidden: it's calculated from the `clk_div` you choose.

```rust
    lstimer0.configure(ledc::timer::config::Config {
        clock_source: ledc::timer::LSClockSource::APBClk,
        frequency: Rate::from_khz(24),
        duty: ledc::timer::config::Duty::Duty5Bit,  // choose
    }).unwrap();
```

Your goal is to get as close to the desired frequency as possible, using this formula:

> duty_resolution_bits = log2( clock_freq / ( desired_pwm_freq * clk_div ) ), where
> `clk_div`=1..1023.

* The lowest resolution is achieved with maxing out `clk_div=1023`.
* The highest resolution is achieved with `clk_div=1` (no division).

For 24 Khz: low=2, high=12. Choose any number in between.

## Example

Initialize:

```rust
// Init PWM.
// Note that in `esp-hal` this is an unstable feature:
// $ cargo add esp-hal unstable
// Currently only supports fixed-frequency output.
let mut ledc = ledc::Ledc::new(peripherals.LEDC);

// Set global slow clock source. (Note: high-speed PWM is not available on ESP32-C3/C6)
ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk);

// Get a new timer
let mut lstimer0 = ledc.timer::<ledc::LowSpeed>(ledc::timer::Number::Timer0);
use ledc::timer::TimerIFace;  // Bring in: .configure()
lstimer0.configure(ledc::timer::config::Config {
    clock_source: ledc::timer::LSClockSource::APBClk,
    // We'll set the frequency to 24 kHz.
    // > duty_resolution_bits = log2( clock_freq / ( desired_pwm_freq * clk_div ) ), where
    // Solve for `clk_div`=1 (max) and `clk_div`=1023 (min)
    // For 24 Khz: min=2, max=12. Choose any number in between.
    frequency: Rate::from_khz(24),
    duty: ledc::timer::config::Duty::Duty10Bit,
}).unwrap();

// PWM channel. Configure.
// It maps a timer to a GPIO pin.
use ledc::channel::ChannelIFace;  // Bring in: .configure()
let mut channel0 = ledc.channel(ledc::channel::Number::Channel0, onboard_led);
channel0.configure(ledc::channel::config::Config {
    timer: &lstimer0,
    // Duty percentage.
    // 10% => 90% brightness
    // 90% => 10% brightness
    duty_pct: 90,
    // How to drive the pin
    drive_mode: gpio::DriveMode::PushPull,
}).unwrap();
```

Change duty cycle:

```rust
channel0.set_duty(30); // 30%
```

Fade-in, fade-out using hardware support for gradual duty cycle changes:

```rust
// PWM has `start_duty_fade()`: gradually changes from one duty cycle percentage to another.
// Fade in
channel0.start_duty_fade(75, 100, 500).unwrap();
while channel0.is_duty_fade_running() {} // wait

// Fade out
channel0.start_duty_fade(100, 75, 500).unwrap();
while channel0.is_duty_fade_running() {} // wait
```