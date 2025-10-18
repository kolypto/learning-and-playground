#![no_std]
#![no_main]
#![deny(clippy::mem_forget)]
use {esp_backtrace as _, esp_println as _};
// use {esp_println as _};
esp_bootloader_esp_idf::esp_app_desc!();
use defmt::info;
use core::fmt::Debug;

use esp_hal::{
    time::Rate,
    clock::CpuClock,
    delay::Delay,
    gpio::{Output, Level, OutputConfig},
    spi::{
        Mode as SpiMode,
        master::{Spi, Config as SpiConfig},
    },
    main,
};

use embedded_hal_bus::{
    spi::ExclusiveDevice,
};
use embedded_sdmmc::{
    SdCard, VolumeIdx, VolumeManager
};


// The sdmmc driver needs a time source to get the current time to handle ctime/mtime for files
#[derive(Default)]
pub struct DummyTimesource();
impl embedded_sdmmc::TimeSource for DummyTimesource {
    // In theory you could use the RTC of the rp2040 here,
    // if you had any external time synchronizing device.
    // See: https://esp32.implrust.com/sdcard/write-sdcard.html
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}



#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    let delay = Delay::new();

    // Init SPI bus
    let spi_bus = Spi::new(
        peripherals.SPI2,
        SpiConfig::default()
            // SD Cards require intial SCK=100 kHz .. 400 kHz
            .with_frequency(Rate::from_khz(400))
            .with_mode(SpiMode::_0),
        ).unwrap()
        .with_sck(peripherals.GPIO4)
        .with_miso(peripherals.GPIO5)
        .with_mosi(peripherals.GPIO6)
        ;

    // Init time source for SDcard file mtimes
    let sd_timer = DummyTimesource{};

    // Device, Driver, with its Chip Select
    // NOTE: that `SdCard` defers initialization until it's first used.
    // NOTE: CS is active-LOW, so we start with HIGH: not selected
    let cs = Output::new(peripherals.GPIO7, Level::High, OutputConfig::default());
    let spi_dev = ExclusiveDevice::new(spi_bus, cs, delay).unwrap();
    let sdcard = SdCard::new(spi_dev, delay);

    // Get SD card info
    let sd_size = sdcard.num_bytes().expect("Get SD card size");
    let sd_type = sdcard.get_card_type().expect("Get SD card type");
    defmt::info!("SD size: {}", sd_size);
    defmt::info!("SD type: {}", sd_type);

    // Speed up after init
    // TODO: (reconfigure SPI to 10-20 MHz here)
    sdcard.spi(|spi| spi.bus_mut().apply_config(
        &SpiConfig::default().with_frequency(Rate::from_mhz(12))
    )).expect("Switch to higher speeds");

    // Volume Manager: find a partition: volume #0
    let volume_mgr = VolumeManager::new(sdcard, sd_timer);
    let volume0 = volume_mgr.open_volume(VolumeIdx(0)).unwrap();
    let root_dir = volume0.open_root_dir().unwrap();

    // Write to file
    let mut my_file = root_dir.open_file_in_dir(
        "FERRIS.TXT",
        embedded_sdmmc::Mode::ReadWriteCreateOrTruncate,
    ).expect("Open file for writing");
    let line = "Hello, Ferris!";
    match my_file.write(line.as_bytes()) {
        Ok(()) => my_file.flush().expect("Write to file"),
        Err(e) => defmt::error!("Failed to write to file")
    }
    my_file.close().expect("Close file");

    // Read the file
    let mut my_file = root_dir
        .open_file_in_dir("FERRIS.TXT", embedded_sdmmc::Mode::ReadOnly)
        .expect("Open file for reading");
    while !my_file.is_eof() {
        let mut buffer = [0u8; 32];
        if let Ok(n) = my_file.read(&mut buffer) {
            for b in &buffer[..n] {
                defmt::info!("{}", *b as char);
            }
        }
    }

    loop {
        delay.delay_millis(100);
    }
}
