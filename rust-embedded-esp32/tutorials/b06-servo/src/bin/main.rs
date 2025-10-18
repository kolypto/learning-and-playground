#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();

use log;
use esp_hal::{
    delay::Delay,
    clock::CpuClock, gpio, ledc,
    time::{Duration, Rate},
    rng::Rng,
    main,
};

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    // Peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    let servo_pin = peripherals.GPIO0;

    // PWM
    let mut ledc = ledc::Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk); // nothing works without this line!
    let mut pwm_timer = ledc.timer::<ledc::LowSpeed>(ledc::timer::Number::Timer0);
    let mut pwm_channel = ledc.channel(ledc::channel::Number::Channel0, servo_pin);
    use ledc::timer::TimerIFace;  // brings: .configure()
    use ledc::channel::ChannelIFace;  // brings: .configure()

    // Configure timer, channel.
    pwm_timer.configure(ledc::timer::config::Config {
        clock_source: ledc::timer::LSClockSource::APBClk,
        duty: ledc::timer::config::Duty::Duty12Bit,
        frequency: Rate::from_hz(50),
    }).unwrap();
    pwm_channel.configure(ledc::channel::config::Config {
        timer: &pwm_timer,
        duty_pct: 0,
        drive_mode: gpio::DriveMode::PushPull,
    }).unwrap();

    // We need to control the pulse width using duty cycle.
    // Max duty cycle depends on the duty resolution bits we've configured.
    use embedded_hal::pwm::SetDutyCycle;
    let max_duty_cycle = pwm_channel.max_duty_cycle();

    // Convert our servo's extreme values into duty cycle range
    const SERVO_MIN: Duration = Duration::from_micros(600);
    const SERVO_MAX: Duration = Duration::from_micros(2575);
    const PWM_FREQ: Rate = Rate::from_hz(50);
    let min_duty: u16 = (max_duty_cycle as f64 * (SERVO_MIN.as_micros() as f64) / (PWM_FREQ.as_duration().as_micros() as f64)) as u16;
    let max_duty: u16 = (max_duty_cycle as f64 * (SERVO_MAX.as_micros() as f64) / (PWM_FREQ.as_duration().as_micros() as f64)) as u16;

    log::info!("max_duty_cycle={max_duty_cycle}");
    log::info!("min_duty={min_duty}");
    log::info!("max_duty={max_duty}");

    // Go
    let delay = Delay::new();
    let rng = Rng::new();
    loop {
        // Gen random angle
        let angle = (rng.random() % 180) as u8;

        // Go to angle
        let duty = duty_from_angle(angle, min_duty, max_duty);
        pwm_channel.set_duty_cycle(duty).unwrap();

        // Sleep
        delay.delay_millis(500);
    }
}


// Angle => Duty perc
// 0 .. 180
fn duty_from_angle(deg: u8, min_duty: u16, max_duty: u16) -> u16 {
    let k = deg as f32 / 180.0;  // 0..100
    min_duty + ((max_duty-min_duty) as f32 * k) as u16
}
