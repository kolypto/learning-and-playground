#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::panic;

use esp_backtrace as _;
use esp_println::{print, println};
use esp_hal::{
    clock::CpuClock,
    time::Rate,
    delay::Delay,
    main,
    // DMA
    dma::{DmaRxBuf, DmaTxBuf},
    dma_buffers,
    spi::{
        master::{Config, Spi},
        Mode,
    },
};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Select pins
    //
    // NOTE: To make testing easy, connect GPIO4 to GPIO2.
    // This way the data we send is also the data we receive.
    let sclk = peripherals.GPIO0;  // Clock signal
    let miso = peripherals.GPIO2;  // Master -> Slave
    let mosi = peripherals.GPIO4;  // Slave -> Master
    let cs = peripherals.GPIO5;    // Chip select. HIGH=Ignore, LOW=select

    // DMA Init
    // The DMA peripheral can perform memory transfers in parallel to the work of the processor:
    // i.e. while your program is doing something else.
    // The peripheral needs two buffers allocation in the ESP memory: 3200 bytes in our case.
    // It also needs some space for descriptors (linked list that DMA uses internally for transfer).
    let dma_channel = peripherals.DMA_CH0;
    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(32000);
    let mut dma_rx_buf = DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let mut dma_tx_buf = DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();

    // Configure SPI to use DMA.
    // We call `.with_dma()` on the SPI driver to make it use DMA
    let mut spi = Spi::new(
        // Note that SPI is a peripheral. ESP32 has SPI0, SPI1, SPI2
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(100))  // SPI frequency for DMS
            .with_mode(Mode::_0),  // SPI Mode: CPOL=0, CPHA=0
        )
        .unwrap()
        .with_sck(sclk)  // pin
        .with_mosi(mosi) // pin
        .with_miso(miso) // pin
        .with_cs(cs)     // pin
        .with_dma(dma_channel);  // use DMA

    let delay = Delay::new();

    // Populate the tx_buffer with data to send
    dma_tx_buf.as_mut_slice().fill(0x42);

    loop {
        // Initiate DMA transfer.
        // It will proceed in the background.
        //
        // Note that the buffers and the driver *move* into the `transfer` object
        // and are inaccessible during the transfer.
        // We'll get them back when the transfer is over.
        println!("Starting DMA transfer: {} bytes", dma_tx_buf.len());
        let transfer = match spi
            .transfer(dma_rx_buf.len(), dma_rx_buf, dma_tx_buf.len(), dma_tx_buf)
            .map_err(|e| e.0) {
            Ok(t) => t,
            Err(e) => {
                println!("DMA transfer failed");
                panic!("DMS error: {e:?}");
            }
        };
        // TODO: I don't know why Wokwi stops here. Maybe DMA is not implemented?
        println!("Started DMA transfer");

        // ... The CPU can do other things while the transfer is taking place ... //

        // ❗ You may think that using DMA is *always* preferable.
        // In fact, not: Setting up a DMA transfer consumes more CPU cycles than setting up a blocking transfer.
        // Especially if the amount of data is small, or if the CPU does nothing else besides waiting,
        // it's preferable to use a blocking transfer.

        // Let's wait
        while !transfer.is_done() {
            print!(".");
            delay.delay_millis(100);
        }
        // if the transfer isn't completed this will block
        (spi, (dma_rx_buf, dma_tx_buf)) = transfer.wait();

        // Done transfer
        println!(
            "Received {:x?} .. {:x?}",
            &dma_rx_buf.as_slice()[..10],
            &dma_rx_buf.as_slice().last_chunk::<10>().unwrap()
        );

        // Pause
        delay.delay_millis(2500);
    }
}
