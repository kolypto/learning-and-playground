use anyhow::{Result, bail};

use esp_idf_svc::{
    // Manages HTTP connections.
    // See: embedded_svc::http::client::Client
    http::client::{EspHttpConnection, Configuration},
};
use embedded_svc::http::{Headers, Method, client::Client};

pub fn get_page(url: &str) -> Result<(u64, String)> {
    // Create a new ESP Http Connection.
    // Wrap it with embedded_svc's Client.
    let connection = EspHttpConnection::new(&Configuration{
        // By default, only unencrypted HTTP is available.
        // Here we enable the use of certificates:
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;
    let mut client = Client::wrap(connection);

    // Make a request
    let request = client.request(Method::Get, url, &[("accept", "text/plain")])?;
    let response = request.submit()?;

    // Status code
    let status = response.status();
    let content_len = response.content_len().unwrap_or(0);
    println!("Response code: {}\n", status);
    log::info!("Content-length: {content_len}");

    // Bad code?
    if !(200..299).contains(&status) {
        bail!("Bad status code: {status}");
    }

    // Good code.
    // Read response data into a buffer.
    let mut buf = [0_u8; 256];
    let mut offset = 0;
    let mut total = 0;
    let mut reader = response;
    loop {
        match reader.read(&mut buf[offset..]) {
            // EOF: size = 0
            Ok(0) => break,
            // Read N bytes
            Ok(n) => {
                offset += n;
                total += n;
                if offset >= buf.len() {
                    break; // buffer full
                }
            }
            // Fail
            Err(e) => return Err(e.into()),
        }
    }

    // Convert into UTF-8 Rust string.
    // NOTE: there's no guarantee that we ended up with a valid UTF-8 sequence: in fact, it is likely
    // that our chunked reader has broken up some sequences.
    // This edge case has to be handled.
    match str::from_utf8(&buf[..total]) {
        Ok(text) => Ok((content_len, text.to_string())),  //-> String
        Err(err) => {
            // The buffer has Incomplete UTF-8 data. Let's extract the valid part.
            let valid_up_to = err.valid_up_to();
            Ok((
                content_len,
                unsafe {
                    str::from_utf8_unchecked(&buf[..valid_up_to]).to_string()
                }
            ))
        }
    }
}