#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use b03_buzzer::{music, nokia};
use esp_backtrace as _;
esp_bootloader_esp_idf::esp_app_desc!();

use log::info;
use esp_hal::{
    time::{Duration, Instant, Rate},
    clock::CpuClock,
    gpio,
    ledc,
    main,
};

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Buzzer pin
    let mut buzzer_pin = peripherals.GPIO8;

    // Active buzzer: just give it voltate.
    // NOTE: comment it out because it consumes the pin
    let mut buzzer = gpio::Output::new(buzzer_pin, gpio::Level::High,
        // Use OpenDrain if GPIO is connected to the transistor's base and there's external pull up.
        // Oherwise it will give a continuous buzz and that's it.
        gpio::OutputConfig::default()
        .with_drive_mode(gpio::DriveMode::OpenDrain)
    );
    for i in 1..30 {
        buzzer.toggle();
        busy_wait(Duration::from_millis(i*5));
    }


    // Init LEDC
    let mut ledc = ledc::Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(ledc::LSGlobalClkSource::APBClk); // nothing works without this line!

    // Let's play a melody
    let song = music::Song::new(nokia::TEMPO);
    for (note, duration_type) in nokia::MELODY {
        // Get music note
        let note_duration = song.calc_note_duration(duration_type) as u64;
        let pause_duration = note_duration / 10; // 10% of note_duration
        if note == music::REST {
            busy_wait(Duration::from_millis(note_duration));
            continue;
        }
        let freq = Rate::from_hz(note as u32);

        // Prepare PWM
        // Note that we can't just keep re-using `buzzer` because it's consumed by the function.
        // We do reborrow()
        let buzzer = buzzer_pin.reborrow();
        let mut pwm_timer = ledc.timer::<ledc::LowSpeed>(ledc::timer::Number::Timer0);
        let mut pwm_channel = ledc.channel(ledc::channel::Number::Channel0, buzzer);
        use ledc::timer::TimerIFace;  // brings: .configure()
        use ledc::channel::ChannelIFace;  // brings: .configure()

        // Configure timer, channel.
        pwm_timer.configure(ledc::timer::config::Config {
            clock_source: ledc::timer::LSClockSource::APBClk,
            duty: ledc::timer::config::Duty::Duty10Bit,
            frequency: freq, // play the frequency
        }).unwrap();
        pwm_channel.configure(ledc::channel::config::Config {
            timer: &pwm_timer, // use the timer
            duty_pct: 50,
            drive_mode: gpio::DriveMode::PushPull,
        }).unwrap();

        // Play
        busy_wait(Duration::from_millis(note_duration - pause_duration)); // play 90%

        // Pause.
        // Disable PWM by setting duty=0: effectively, no signal
        pwm_channel.set_duty(0).unwrap();
        busy_wait(Duration::from_millis(pause_duration)); // Pause for 10%
    }

    loop{
        busy_wait(Duration::from_millis(100));
    }
}

// Blocking delay: burns CPU cycles until `duration` passes.
fn busy_wait(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
