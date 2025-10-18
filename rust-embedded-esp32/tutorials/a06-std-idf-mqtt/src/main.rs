use anyhow::Result;

// Our libraries
use a04_std_idf_http_client::{
    blinky_led,
    wifi,
};

// IDF
use esp_idf_hal::{
    prelude::*,
    temp_sensor,
    // delay::FreeRtos,
};
use esp_idf_svc::{
    mqtt::client::{EspMqttClient, Details, MqttClientConfiguration},
};



// Drivers
struct Drivers<'d,
    // It's nice to have all the pinout in one place, isn't it?
    StatusLedPin: gpio::Pin = gpio::Gpio8,
> {
    status_led: blinky_led::BlinkyLed<'d, StatusLedPin, gpio::Output>,
    internal_temp: esp_idf_hal::temp_sensor::TempSensorDriver<'d>,
}


fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let sysloop = EspSystemEventLoop::take()?;
    let _nvs = EspDefaultNvsPartition::take()?;

    // Peripherals
    let config = CONFIG;
    let mut drv = Drivers::new()?;

    // WiFi
    let _wifi = wifi::new(config.wifi_ssid, config.wifi_psk, drv.modem, sysloop)?;
    drv.status_led.blink(3, Duration::from_millis(100), Duration::from_millis(200))?;

    // MQTT
    // NOTE: start a minimal MQTT server:
    //  $ docker run --rm -it -v ./nats-server.conf:/nats-server.conf -p 1883:1883 nats
    let mqtt_config = MqttClientConfiguration::default();
    let mut client = EspMqttClient::new_cb(&broker_url, &mqtt_config, move |message| {
        // MQTT messages receiver
        match message.payload() {
            Received { data, details, .. } => process_message(data, details, &mut led),
            Error(e) => warn!("Received error from MQTT: {:?}", e),
            _ => info!("Received from MQTT: {:?}", message_event.payload()),
        }
    })?;

    // Publish a message
    let payload: &[u8] = &[];
    client.enqueue(&hello_topic(UUID), QoS::AtLeastOnce, true, payload)?;

    loop {
        sleep(Duration::from_secs(1));
        let temp = temp_sensor
            .measure_temperature(PowerMode::NormalMode, &mut delay)
            .unwrap()
            .as_degrees_celsius();
        // 3. publish CPU temperature
        client.enqueue(
            &mqtt_messages::temperature_data_topic(UUID),
            QoS::AtLeastOnce,
            false,
            &temp.to_be_bytes() as &[u8],
        )?;
    }

    loop {
        FreeRtos::delay_ms(100);
    }
}

fn process_message(data: &[u8], details: Details, led: &mut WS2812RMT) {
    match details {
        Complete => {
            info!("{:?}", data);
            let message_data: &[u8] = data;
            if let Ok(ColorData::BoardLed(color)) = ColorData::try_from(message_data) {
                info!("{}", color);
                if let Err(e) = led.set_pixel(color) {
                    error!("Could not set board LED: {:?}", e)
                };
            }
        }
        _ => {}
    }
}

impl <'d> Drivers<'d> {
    fn new() -> Result<Self> {
        // Take the peripherals.
        // Note that the value is moved, and it was a singleton.
        // Now no one else can use the peripherals: only through us.
        let peripherals = esp_idf_hal::peripherals::Peripherals::take()?;

        // Drivers
        Ok(Self {
            status_led: {
                let mut status_led_output = PinDriver::output(peripherals.pins.gpio8)?;
                blinky_led::BlinkyLed::new(status_led_output)?
            },
            internal_temp: {
                let mut sensor = temp_sensor::TempSensorDriver::new(&temp_sensor::TempSensorConfig::default(), peripherals.temp_sensor)?;
                sensor.enable()?;
                sensor
            },
        })
    }
}


#[toml_cfg::toml_config]
pub struct Config { // cfg.toml
    #[default("Wokwi-GUEST")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
    #[default("")]
    mqtt_broker_url: &'static str,
}

