use defmt;
use anyhow::Result;
use core::sync::atomic::{AtomicU8, Ordering};
use core::cell::RefCell;
use heapless::Deque;


use esp_hal::gpio;
use esp_radio::{
    ble::controller::BleConnector,
};
use trouble_host::prelude::*;

use embassy_executor::Spawner;
use embassy_time::{Timer, Duration};
use embassy_futures::join::join;

// The specific BT device to find.
// Buzzer will click louder as you approach the device and the signal gets better.
//
// NOTE: BT addresses are Little-Endian so written BACKWARDS!
const TARGET: &[u8] = &[0x41, 0xFE, 0x83, 0x1C, 0x8F, 0x52];

// Max number of simultaneous connections to this server
const CONNECTIONS_MAX: usize = 1;
// Max number of communication channels (?)
const L2CAP_CHANNELS_MAX: usize = 1;
// The number of commands that may wait for responses
const BT_CONTROLLER_SLOTS: usize = 20;

pub async fn run_tasks(
    spawner: &Spawner,
    bt_transport: BleConnector<'static>,
    buzzer: gpio::Output<'static>
) -> Result<()> {
    // Init Bluetooth. HCI controller. Resources (mem). Stack.
    // Generated code.
    let ble_controller = ExternalController::<_, BT_CONTROLLER_SLOTS>::new(bt_transport);
    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> = HostResources::new();
    let stack = trouble_host::new(ble_controller, &mut resources);

    // Set random BT address: 48 bit. Like a MAC address.
    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    let stack = stack.set_random_address(address);

    // Build stack
    let Host { central, mut runner, ..  } = stack.build();

    // Finally: spawn tasks
    spawner.spawn(task_buzzer(buzzer))?;
    spawner.spawn(task_scan_bluetooth())?;

    // Prepare scanner
    // Requires: $ cargo add trouble-host --features scan
    let mut scanner = Scanner::new(central);
    let scan_config = ScanConfig{
        active: true,
        phys: PhySet::M1,
        interval: Duration::from_secs(3),
        window: Duration::from_secs(3),
        ..Default::default()
    };

    // BT events handler
    let handler = Printer { seen: RefCell::new(Deque::new()) };
    let handler = BleFoundDeviceHandler {};


    // Scan continuously: join() two tasks
    // - run_with_handler() to process BT events: i.e. found device
    // - scan, repeat, scan
    let _ = join(
        runner.run_with_handler(&handler),
        async {
            loop {
                // Scan. Then wait for the exact scan time.
                defmt::info!("Scan");
                let mut session = scanner.scan(&scan_config).await.unwrap();
                Timer::after(scan_config.interval).await;
            }
        }
    ).await;

    Ok(())
}

// Task: Bluetooth Scanner
#[embassy_executor::task]
pub async fn task_scan_bluetooth() -> ! {
    loop {
        Timer::after_secs(2).await;
    }
}

// How close is the target? 1..255.
// Value=0 means "not found".
// Defines the frequency of clicks.
static PROXIMITY: AtomicU8 = AtomicU8::new(0);

// Task: Buzzer.
// Produces period "beeps" as you approach the target.
#[embassy_executor::task]
pub async fn task_buzzer(mut buzzer: gpio::Output<'static>) -> ! {
    loop {
        // Delay between clicks
        let prox = PROXIMITY.load(Ordering::Relaxed);
        let delay = Duration::from_millis(4*(prox as u64));

        // Not found?
        if prox == 0 {
            Timer::after_secs(1).await;
            continue;
        }

        // Click
        buzzer.toggle();
        Timer::after_micros(750).await;
        buzzer.toggle();

        // Delay
        Timer::after(delay).await;
    }
}


// BT event handler for found devices.
// The original handler: prints only newly found devices; uses Dequeue for deduplication.
struct Printer {
    seen: RefCell<Deque<BdAddr, 128>>,
}
impl EventHandler for Printer {
    fn on_adv_reports(&self, mut it: LeAdvReportsIter<'_>) {
        let mut seen = self.seen.borrow_mut();
        while let Some(Ok(report)) = it.next() {
            if seen.iter().find(|b| b.raw() == report.addr.raw()).is_none() {
                defmt::info!("discovered: {:?} rssi={:?}", report.addr, report.rssi);
                if seen.is_full() {
                    seen.pop_front();
                }
                seen.push_back(report.addr).unwrap();
            }
        }
    }
}


// BT event handler for found devices.
// Out handler updates the PROXIMITY value
struct BleFoundDeviceHandler {}
impl EventHandler for BleFoundDeviceHandler {
    fn on_adv_reports(&self, mut it: LeAdvReportsIter<'_>) {
        while let Some(Ok(report)) = it.next() {
            if report.addr.raw() == TARGET {
                let prox = rssi_to_proximity(report.rssi);
                PROXIMITY.store(prox, Ordering::Relaxed);
                defmt::info!("TARGET: {:?} rssi={:?} prox={:?}", report.addr, report.rssi, prox);
            } else {
                defmt::info!("another: {:?} rssi={:?}", report.addr, report.rssi);
            }
        }
    }
}

// RSSI -> Proximity. Good RSSI: >-30 Bad RSSI: <-90
// Map to: 1..255. Logarithmic.
fn rssi_to_proximity(rssi: i8) -> u8 {
    let clamped = rssi.clamp(-90, -30);

    // Exponential decay: closer signals are much stronger
    // Map [-30, -90] with exponential curve
    let normalized = (-30 - clamped) as f32 / 60.0; // [0.0, 1.0]

    // Apply exponential curve (adjust exponent to taste)
    use libm;
    let curved = libm::powf(normalized, 2.5); // More aggressive curve

    let proximity = (curved * 254.0) + 1.0;
    proximity as u8
}
