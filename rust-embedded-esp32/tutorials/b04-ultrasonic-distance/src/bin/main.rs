#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();

use log;
use esp_hal::{
    delay::Delay,
    rtc_cntl::Rtc,
    clock::CpuClock, gpio, ledc,
    time::{Duration, Rate},
    main,
};

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // PWM for the buzzer
    let mut buzzer_pin = peripherals.GPIO4;
    let mut ledc = ledc::Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk); // nothing works without this line!

    // Ultrasonic sensor:
    // - TRIG pin: send a short pulse (10us) to trigger a reading
    // - ECHO pin: will respond with a continuous pulse. It's length is proportional to the distance.
    //   The initial state of this pin will be set to Pull Down to ensure it starts in the low state.
    let ultrasonic_trig_pin = peripherals.GPIO0;
    let mut ultrasonic_trig = gpio::Output::new(ultrasonic_trig_pin, gpio::Level::Low, gpio::OutputConfig::default());
    let ultrasonic_echo_pin = peripherals.GPIO1;
    let ultrasonic_echo = gpio::Input::new(ultrasonic_echo_pin, gpio::InputConfig::default()
        .with_pull(gpio::Pull::Down));

    // RTC time and delay
    let delay = Delay::new();
    let rtc = Rtc::new(peripherals.LPWR);

    // Keep measuring
    loop {
        // Send a TRIG to the ultrasonic module.
        // The datasheet requires it to be at least 10us long.
        ultrasonic_trig.set_low();
        delay.delay_micros(2);
        ultrasonic_trig.set_high();
        delay.delay_micros(10);
        ultrasonic_trig.set_low();

        // Now measure the response pulse width:
        // 1. Wait while it's low. Then record the time.
        // 2. Wait while it's high. Then record the time.
        while ultrasonic_echo.is_low() {}
        let time1 = rtc.current_time_us();
        while ultrasonic_echo.is_high() {}
        let time2 = rtc.current_time_us();
        let pulse_width = Duration::from_micros(time2 - time1);

        // The pulse width tells us how long it took for the ultrasonic waves to travel
        // to an obstacle and return. Since the pulse represents the round-trip time,
        // we divide it by 2 to account for the journey to the obstacle and back.
        let distance_cm = (pulse_width.as_micros() as f64 * 0.0343) / 2.0;  // speed of sound / 2

        // Convert distance to frequency
        const MIN_DISTANCE: f64 = 10.0;
        const MAX_DISTANCE: f64 = 200.0;
        const MIN_FREQ: u32 = 700;
        const MAX_FREQ: u32 = 7000;
        let freq = Rate::from_hz(match distance_cm {
            n if n < MIN_DISTANCE => MIN_FREQ,
            n if n > MAX_DISTANCE => MAX_FREQ,
            n => MIN_FREQ + (MAX_FREQ as f64 * n/MAX_DISTANCE) as u32,
        });
        let duty = match distance_cm {
            n if n > MAX_DISTANCE => 0,
            _ => 10,
        };
        log::info!("distance={distance_cm}cm freq={freq} duty={duty}");

        // Configure PWM to give us SOUND!
        let buzzer = buzzer_pin.reborrow();
        let mut pwm_timer = ledc.timer::<ledc::LowSpeed>(ledc::timer::Number::Timer0);
        let mut pwm_channel = ledc.channel(ledc::channel::Number::Channel0, buzzer);
        use ledc::timer::TimerIFace;  // brings: .configure()
        use ledc::channel::ChannelIFace;  // brings: .configure()
        pwm_timer.configure(ledc::timer::config::Config {
            clock_source: ledc::timer::LSClockSource::APBClk,
            duty: ledc::timer::config::Duty::Duty7Bit,
            frequency: freq,
        }).unwrap();
        pwm_channel.configure(ledc::channel::config::Config {
            timer: &pwm_timer,
            duty_pct: duty,
            drive_mode: gpio::DriveMode::PushPull,
        }).unwrap();

        // Sleep a bit. Don't measure too often.
        // This is also a requirement from the ultrasonic module: interval between trigger pulses.
        delay.delay_millis(60);
    }
}
