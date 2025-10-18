use embassy_time::{Duration, Timer};
use esp_hal::gpio::Output;

use core::sync::atomic::{AtomicBool, Ordering};

// Primitive shared-memory communication:
// 1. webserver will set the value
// 2. LED task will read it
pub static LED_STATE: AtomicBool = AtomicBool::new(false);

// Task: toggle LED, controlled by static `LED_STATE`
#[embassy_executor::task]
pub async fn led_task(mut led: Output<'static>) {
    // NOTE: reading a GPIO state is as easy as reading a mem region.
    // Therefore we don't have to keep this bool here: just read current state and compare.
    let mut current_led_state: bool = led.is_set_low(); // the LED is inverted
    loop {
        // Read expected LED state
        let set_led_state = LED_STATE.load(Ordering::Relaxed);

        // Set LED. Only if changed.
        match (current_led_state, set_led_state) {
            (true, false) => led.set_high(),
            (false, true) => led.set_low(),
            _ => ()
        }
        current_led_state = set_led_state;

        // Sleep
        Timer::after(Duration::from_millis(50)).await;
    }
}
