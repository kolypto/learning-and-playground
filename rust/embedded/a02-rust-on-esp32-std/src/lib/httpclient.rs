use anyhow::{bail, Result};
use core::str;
use esp_idf_sys;
use embedded_svc::{
    http::{client::Client, Status},
    io::Read,
};
use esp_idf_svc::{
    http::client::{Configuration, EspHttpConnection},
};


/// Download data from an HTTP URL
// The `AsRef<str>` means the function accepts anything that implements the trait: both `&str` and `String`
pub fn get_url(url: impl AsRef<str>) -> Result<()> {
    // Create a client: EspHttpConnection, then Client
    let connection = EspHttpConnection::new(&Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;
    let mut client = Client::wrap(connection);

    // Open a GET request
    let request = client.get(url.as_ref())?;

    // Sed the request
    let response = request.submit()?;
    let status = response.status();
    println!("Response code: {}\n", status);

    // Check the HTTP code
    match status {
        200..=299 => {
            // Read response, buffer size = 256
            let mut buf = [0_u8; 256];
            let mut offset = 0; // offset in the buffer
            let mut total = 0;  // total number of bytes read
            let mut reader = response;

            // Keep reading
            loop {
                // Read into the buffer, starting at `offset`
                if let Ok(size) = Read::read(&mut reader, &mut buf[offset..]) {
                    // Read nothing? stop reading.
                    if size == 0 {
                        break;
                    }
                    total += size;

                    // Try converting the bytes into UTF-8 and print
                    let size_plus_offset = size + offset;
                    match str::from_utf8(&buf[..size_plus_offset]) {
                        Ok(text) => {
                            // Print
                            print!("{}", text);
                            // Empty the buffer
                            offset = 0;
                        },
                        Err(error) => {
                            let valid_up_to = error.valid_up_to();
                            unsafe {
                                print!("{}", str::from_utf8_unchecked(&buf[..valid_up_to]));
                            }

                            // Move bytes in the buffer
                            buf.copy_within(valid_up_to.., 0);
                            offset = size_plus_offset - valid_up_to;
                        }
                    }
                }
            }
            println!("Total: {} bytes", total);
        }
        _ => bail!("Unexpected response code: {}", status),
    }

    Ok(())
}
