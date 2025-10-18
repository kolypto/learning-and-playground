use anyhow::Result;
use core::time::Duration;
use esp_idf_hal::{
    gpio::OutputPin,
    peripheral::Peripheral,
    rmt::{config::TransmitConfig, FixedLengthSignal, PinState, Pulse, RmtChannel, TxRmtDriver},
};

pub use rgb::RGB8;

// Driver for the WS2812 LED.
// Using the RMT (Remote Control) channel:
//   can generate many periodic sequences with minimal CPU intervention.
//   That is, send 10101001 binary sequences with specific lengths.
// See also:
// - https://crates.io/crates/ws2812-esp32-rmt-driver
// - https://crates.io/crates/ws2812-spi
pub struct WS2812RMT<'a> {
    // RTM driver
    rmt: TxRmtDriver<'a>,

    // Pre-configured pulse lengths for WS2812
    pulse: PulseConfig,
}

// Pre-configured pulse lengths for WS2812
struct PulseConfig {
    // Transmit 0: short HIGH + long LOW
    t0h: Pulse,
    t0l: Pulse,
    // Transmit 1: short LOW + long HIGH
    t1h: Pulse,
    t1l: Pulse,
}

// This is mostly copied from the std training:
//   https://github.com/esp-rs/std-training/blob/main/common/lib/rgb-led/src/lib.rs
impl<'d> WS2812RMT<'d> {
    // Init: led + RMT peripheral
    pub fn new(
        led: impl Peripheral<P = impl OutputPin> + 'd,
        rmt: impl Peripheral<P = impl RmtChannel> + 'd,
    ) -> Result<Self> {
        // RMT works with ticks: every pulse's length is given in tick counts.
        // ESP32-C3 has an internal RMT clock running at 80 Mhz
        // Clock divider: slow down the internal RMT clock (80 Mhz on ESP32-C3) to get the timing resolution you need.
        // This is because RMT works with ticks, not with milliseconds.
        // So with clk_div=2: one tick = 2/80 Mhz = 0.025μs = 25ns.
        // Because pulse length is a u16, you can generate pulses with lengths 25ns..1600μs
        // With Pulse length in ticks being a u16, you can generate signals between 25ns..819μs
        // Btw, the range for RMT:
        // - min: clk_div=1  : 12.5ns .. 409μs
        // - max: clk_div=255:    3μs .. 104ms
        let config = TransmitConfig::new().clock_divider(2);

        // RTM Tx driver
        let rmt_tx_driver = TxRmtDriver::new(rmt, led, &config)?;

        // Get the speed of the internal clock: used for calculating the number of ticks
        let ticks_hz = rmt_tx_driver.counter_clock()?;
        log::info!("hz={ticks_hz}");


        // Pre-calculate the actual pulse lengths.
        // They depend on the clock freq and the clk_div clock divider factor.
        let pulse_config = PulseConfig{
            t0h: Pulse::new_with_duration(ticks_hz, PinState::High, &ns(400))?,
            t0l: Pulse::new_with_duration(ticks_hz, PinState::Low , &ns(850))?,
            t1h: Pulse::new_with_duration(ticks_hz, PinState::High, &ns(800))?,
            t1l: Pulse::new_with_duration(ticks_hz, PinState::Low , &ns(450))?,
        };

        // RMT Tx driver, configured.
        Ok(Self {
            rmt: rmt_tx_driver,
            pulse: pulse_config,
        })
    }

    // Sed the color of exactly one pixel.
    pub fn set_pixel(&mut self, rgb: RGB8) -> Result<()> {
        // Color: convert to GRB encoding
        let color: u32 = ((rgb.g as u32) << 16) | ((rgb.r as u32) << 8) | rgb.b as u32;

        // Prepare the lengths: 24 bits of data, encoded.
        let mut signal = FixedLengthSignal::<24>::new();
        let p = &self.pulse;
        for i in (0..24).rev() {
            // Get the bit
            // TODO: More efficiently: shift "color >> 1" every iteration, destructuring the value
            let bit = (color & 1<<i) != 0;
            // Convert to a pair of Pulses: level + duration
            let (high_pulse, low_pulse) = if bit { (p.t1h, p.t1l) } else { (p.t0h, p.t0l) };
            // Add to signal
            signal.set(23 - i as usize, &(high_pulse, low_pulse))?;
        }

        // Send the signal
        self.rmt.start_blocking(&signal)?;

        // Done
        Ok(())
    }

    // Draw a heart on a 64x64 LED array.
    // This has nothing to do with board test; it's just a fun experiment
    pub fn heart64x64(&mut self) -> Result<()> {
        // NOTE: This HUGE array requires a bigger stack.
        // Size: 64 * 24 = 1.5K
        // Better allocate it in the memory
        const R: RGB8 = RGB8::new(100,0,0);
        const B: RGB8 = RGB8::new(0,0,0);
        const COLORS: [RGB8; 64] = [
            B,R,R,B,B,R,R,B,
            R,B,B,R,R,B,B,R,
            R,B,B,B,B,B,B,R,
            R,B,B,B,B,B,B,R,
            B,R,B,B,B,B,R,B,
            B,B,R,B,B,R,B,B,
            B,B,B,R,R,B,B,B,
            B,B,B,B,B,B,B,B,
        ];

        // Contruct signal
        const SIGNAL_LEN: usize = COLORS.len() * 24;  // 24 bits per LED
        let mut signal = FixedLengthSignal::<SIGNAL_LEN>::new();
        let p = &self.pulse; // shortcut
        for (j, rgb) in COLORS.iter().enumerate() {
            // Color: convert to GRB encoding
            let color: u32 = ((rgb.g as u32) << 16) | ((rgb.r as u32) << 8) | rgb.b as u32;

            // Prepare the lengths: 24 bits of data, encoded.
            for i in 0..24 {
                // Get the bit
                let bit = (color & 1<<(23-i)) != 0;
                // Convert to a pair of Pulses: level + duration
                let (high_pulse, low_pulse) = if bit { (p.t1h, p.t1l) } else { (p.t0h, p.t0l) };
                // Add to signal
                signal.set(j*24 + i as usize, &(high_pulse, low_pulse))?;
            }
        }

        // Send the signal
        self.rmt.start_blocking(&signal)?;

        // Done
        Ok(())
    }
}

fn ns(nanos: u64) -> Duration {
    Duration::from_nanos(nanos)
}
