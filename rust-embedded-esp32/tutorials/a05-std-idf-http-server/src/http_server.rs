use a04_std_idf_http_client::blinky_led;
use anyhow::Result;
use std::{
    sync::{Arc, Mutex}, time::Duration,
};
use esp_idf_hal::{
    io::EspIOError,
    gpio,
};
use esp_idf_svc::{
    http::server::{EspHttpServer, Configuration, Method},
};


// See this moved value, `temp_sensor`?
// Problem: it only lives for the duration of the function: i.e. it gets dropped when func quits.
//  But we have a closure that uses the value. Moreover, it should own it:
//  to be able to use it multiple times; it shouldn't become `FnOnce` either.
// Solution: use `Arc` (Atomic Reference Count) for shared ownership, and
//  `Mutex` for safe, exclusive access to the hardware driver.
// We could also mark `temp_sensor` and the returned `server` with 'a lifetime —
//  but the server's closure is required to be static: `Fn + 'static`.
//  Therefore, all values the static closure uses are also supposed to be static.
pub fn new<RedLedPin, GreenLedPin, BlueLedPin>(
    temp_sensor: esp_idf_hal::temp_sensor::TempSensorDriver<'static>,
    red_led: blinky_led::BlinkyLed<'static, RedLedPin, gpio::Output>,
    green_led: blinky_led::BlinkyLed<'static, GreenLedPin, gpio::Output>,
    blue_led: blinky_led::BlinkyLed<'static, BlueLedPin, gpio::Output>,
) -> Result<EspHttpServer<'static>>
    where
        RedLedPin:    gpio::Pin,
        GreenLedPin:  gpio::Pin,
        BlueLedPin:   gpio::Pin,
{
    // Wrap the owned driver in a `Mutex` for safe mutable access, and an `Arc` for shared ownership.
    let shared_sensor = Arc::new(Mutex::new(temp_sensor));

    // Wrap the leds
    let red_led = Arc::new(Mutex::new(red_led));
    let green_led = Arc::new(Mutex::new(green_led));
    let blue_led = Arc::new(Mutex::new(blue_led));

    // Init the HTTP server
    let mut server = EspHttpServer::new(&Configuration{
        http_port: 80,
        ..Default::default()
    })?;
    server.fn_handler("/", Method::Get, |request| -> core::result::Result<(), EspIOError> {
        // Show index page
        let html = index_html_templated("Hello from ESP32!");
        let mut response = request.into_ok_response()?;
        response.write(html.as_bytes())?;
        Ok(())
    })?;

    // Callback uses the value. It's moved.
    server.fn_handler("/temperature", Method::Get, move |request| -> core::result::Result<(), EspIOError> {
        // Inside the closure, lock the Mutex to gain exclusive access to the driver.
        // It will get unlocked when the function quits: with .drop()
        let sensor = shared_sensor.lock().unwrap();
        let temp = sensor.get_celsius()?;

        // Show temperature page
        let html = index_html_templated(format!("Chip temperature: {:.2}°C", temp));
        let mut response = request.into_ok_response()?;
        response.write(html.as_bytes())?;
        Ok(())
    })?;

    server.fn_handler("/blink", Method::Get, move |request| -> core::result::Result<(), EspIOError> {
        // Blink
        let uri = request.uri();
        if uri.contains("?blink=red") {
            red_led.lock().unwrap().blink(3, Duration::from_millis(100), Duration::from_millis(100));
        }
        if uri.contains("?blink=green") {
            green_led.lock().unwrap().blink(3, Duration::from_millis(100), Duration::from_millis(100));
        }
        if uri.contains("?blink=blue") {
            blue_led.lock().unwrap().blink(3, Duration::from_millis(100), Duration::from_millis(100));
        }

        // Show page
        let html = index_html_templated(r#"
        <ul>
            <li><a href="?blink=red">Red</a></li>
            <li><a href="?blink=green">Green</a></li>
            <li><a href="?blink=blue">Blue</a></li>
        </ul>
        "#);
        let mut response = request.into_ok_response()?;
        response.write(html.as_bytes())?;
        Ok(())
    })?;


    // We need to return the server so that someone owns it.
    Ok(server)
}

fn index_html_templated(content: impl AsRef<str>) -> String {
    format!(r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>
        /* Base styles */
        body {{
            font-size: 12pt;
        }}

        /* Smaller tablets and mobiles */
        @media (max-width: 768px) {{
            body {{
                font-size: 16pt;
            }}
        }}
        </style>
    </head>
    <body>
        {}
    </body>
</html>
    "#, content.as_ref())
}
