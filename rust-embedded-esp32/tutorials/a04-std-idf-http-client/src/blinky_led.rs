use std::time::Duration;

use esp_idf_hal::{
    delay::FreeRtos, gpio::{OutputMode, Pin, PinDriver, Level}
};
use anyhow::{Result};

// Blinky LED controller.
// It's a generic because every PIN is a different type :O
pub struct BlinkyLed<'d, P: Pin, M: OutputMode> {
    output: PinDriver<'d, P, M>,
    inverted: bool
}

impl <'d, P: Pin, M: OutputMode> BlinkyLed<'d, P, M> {
    pub fn new(
        mut output: PinDriver<'d, P, M>,
    ) -> Result<Self> {
        output.set_level(Level::Low);
        Ok(Self{
            output,
            inverted: false,
        })
    }

    pub fn new_inverted(
        mut output: PinDriver<'d, P, M>,
    ) -> Result<Self> {
        output.set_level(Level::High);
        Ok(Self{
            output,
            inverted: true,
        })
    }

    pub fn blink(&mut self, n: u8, on: Duration, off: Duration) -> Result<()>{
        for _ in 0..n {
            self.output.set_level(if self.inverted { Level::Low } else { Level::High });
            FreeRtos::delay_ms(on.as_millis() as u32);

            self.output.set_level(if self.inverted { Level::High } else { Level::Low });
            FreeRtos::delay_ms(off.as_millis() as u32);
        }
        Ok(())
    }
}