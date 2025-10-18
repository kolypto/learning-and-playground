#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use {esp_backtrace as _, esp_println as _};
esp_bootloader_esp_idf::esp_app_desc!();
use defmt;

// Non-blocking
use nb;

// Math for no_std
use libm;

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    analog::adc,
    main,
};


#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default()
        .with_cpu_clock(CpuClock::max()));

    // ADC Input
    // Attenuation: -11dB. Gives us a safe range of 0..3.9V = 1100 * 10 ^ (11/20)
    let ldr_pin = peripherals.GPIO4;
    let mut adc_config = adc::AdcConfig::new();
    let mut pin = adc_config.enable_pin(ldr_pin, adc::Attenuation::_11dB);
    let mut adc1 = adc::Adc::new(peripherals.ADC1, adc_config);

    let delay = Delay::new();
    loop {
        let adc_value: u16 = match nb::block!(adc1.read_oneshot(&mut pin)) {
            Ok(result) => result,
            Err(err) => {
                defmt::error!("err={}", err);
                continue;
            }
        };
        let temperature = adc_value_to_temperature(adc_value, 11);
        defmt::info!("Temperature: {}", temperature);

        delay.delay_millis(100);
    }
}



// Math section

// Our voltage divider circuit
const ADC_VREF: f64 = 2.5; // ESP32 ADC Reference Voltage at the given attenuation (see docs)
const VDV_VDD: f64 = 3.3;   // Input voltage on the divider
const VDV_R1: f64 = 10_000.0;  // Second resistor in the voltage divider

// ADC resolution: 12 bits
// I.e. it maps voltages into 0..4095
const ADC_MAX: i32 = 2 << 12;  // 4095

// The typical B value for the NTC 103 thermistor is 3950.
// The reference temperature is usually 25°C and 10kΩ
const TRM_B_VALUE: f64 = 3950.0;
const TRM_REF_TEMP: f64 = 25.0; // Reference temperature 25°C
const TRM_REF_RES: f64 = 10_000.0; // Thermistor resistance at the Reference Temperature(25°C)


// Calc: measured resistance for RT:
//                         ┌──────────────── GPIO4
//   V_DD ──────[ R_1 ]────┴───[ R_T ]────── GND
//
//   $V_{measured} = V_{ref} * {adc\_value} / {ADC\_MAX}$
//   $R_T = R_1 / (V_{DD}/V_{measured} - 1)$
fn adc_value_to_temperature(adc_value: u16, att_db: u8) -> f64 {
    let V_measured: f64 = ADC_VREF * adc_value as f64 / ADC_MAX as f64;
    let R_thermistor: f64 = VDV_R1 / (VDV_VDD / V_measured - 1.0);
    // defmt::info!("adc_value={}", adc_value);
    // defmt::info!("V_measured={}", V_measured);
    // defmt::info!("R_thermistor={}", R_thermistor);

    // $1/T = 1/T_0 + (1/B) * ln(R/R_0)$
    const K: f64 = 273.15; // Kelvin
    let inv_t = 1.0/(TRM_REF_TEMP + K) + (1.0/TRM_B_VALUE) * libm::log(R_thermistor / TRM_REF_RES);
    return 1.0/inv_t - K;
}