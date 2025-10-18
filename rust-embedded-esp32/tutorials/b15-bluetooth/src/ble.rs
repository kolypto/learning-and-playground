use anyhow::{Result};

use embassy_sync;
use embassy_futures::{
    join::join,
    select::select,
};
use embassy_time::Timer;

use trouble_host::prelude::*;
use esp_radio::{
    ble::controller::BleConnector,
};
use bt_hci::{
    FromHciBytesError, controller::ExternalController, transport::Transport
};


// GATT Server definition
#[gatt_server]
struct Server {
    // One service
    sensor_service: SensorService,
}

/// Battery service
#[gatt_service(uuid = "a9c81b72-0f7a-4c59-b0a8-425e3bcf0a0e")]
struct SensorService {
    // Characteristrics

    // Read-only number
    #[characteristic(uuid = "13c0ef83-09bd-4767-97cb-ee46224ae6db", read, notify)]
    sensor_data: u8,

    // Writable bool
    #[characteristic(uuid = "c79b2ca7-f39d-4060-8168-816fa26737b7", write, read)]
    sensor_settings: bool,
}

// Max number of simultaneous connections to this server
const CONNECTIONS_MAX: usize = 1;
// Max number of communication channels (?)
const L2CAP_CHANNELS_MAX: usize = 1;
// The number of commands that may wait for responses
const BT_CONTROLLER_SLOTS: usize = 20;

// We'll create a BLE server:
// * GATT service
// * two characteristics: one rw, one ro
pub async fn run<'d>(transport: BleConnector<'d>)
    // where T: Transport
    -> Result<()>
{
    // Init Bluetooth. HCI controller. Resources (mem). Stack.
    // Generated code.
    let ble_controller = ExternalController::<_, BT_CONTROLLER_SLOTS>::new(transport);
    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> = HostResources::new();
    let stack = trouble_host::new(ble_controller, &mut resources);

    // Set random BT address: 48 bit. Like a MAC address.
    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    let stack = stack.set_random_address(address);

    // Build stack
    let Host {
        // Get the "peripheral" part.
        mut peripheral,
        runner,
        // We don't need the  "central" part
        // mut central,
        ..
    } = stack.build();

    // Start advertising the GATT service
    let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
        // Peripheral device name
        name: "iBeacon",
        appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
    })).unwrap();

    // Infinite loop. Run both tasks.
    // join() joins the results of two futures
    let _ = join(
        // Task: The BLE stack task: keeps the stack running
        ble_task(runner),
        // Task: Our application logic
        async {
        loop {
            // Advertise: accept incoming connections and handle them.
            // It's like listen() + accept()
            match advertise("impl Rust", &mut peripheral, &server).await {
                // Connection established
                Ok(conn) => {
                    // Handle GATT events: GattEvent::{Read, Write, Other}
                    let a = gatt_events_task(&server, &conn);
                    // Run our application-specific logic during connection
                    let b = custom_task(&server, &conn, &stack);
                    // Run both tasks concurrently until one task ends (usually because of a disconnect)
                    // select() waits for one of two futures to complete
                    select(a, b).await;
                }
                // Connection failed
                Err(e) => {
                    let e = defmt::Debug2Format(&e);
                    panic!("BLE host failed: {:?}", e);
                }
            }
        }
    })
    .await;

    Ok(())
}

// BLE Stack Task
async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            let e = defmt::Debug2Format(&e);
            panic!("BLE core task error: {:?}", e);
        }
    }
}


// Advertise the service: announces its presense to nearby devices
// It broadcasts the device name, available services, and capabilities.
async fn advertise<'values, 'server, C: Controller>(
    // Device name
    name: &'values str,
    // BT Peripheral
    peripheral: &mut Peripheral<'values, C, DefaultPacketPool>,
    // GATT server
    server: &'server Server<'values>,
) -> Result<GattConnection<'values, 'server, DefaultPacketPool>, BleHostError<C::Error>> {
    // Encode advertise data
    let mut advertiser_data = [0; 31];  // bufferw
    let len = AdStructure::encode_slice(
        &[
            // Discoverable + Classic not available
            // BR_EDR: basic rate (BR), enhanced data rate (EDR)
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;

    // Advertise: peripheral.advertise()
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            // Advertisement.
            // Connectable: device can connect
            // Scannable: devices can request additional information (though we provide empty `scan_data` here)
            // Undirected: invite-all, not a specific device
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..len],
                scan_data: &[],
            },
        )
        .await?;

    // Wait for connection, keep advertising
    defmt::info!("BLE: advertising...");
    let conn = advertiser.accept().await?.with_attribute_server(server)?;
    defmt::info!("BLE: connection established");
    Ok(conn)
}


// Handle GATT events
async fn gatt_events_task<P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
) -> Result<(), Error> {
    // Our read-only characteristic.
    // This was generated by the macro when we defined the the #[gatt_ser]
    let sensor_data = server.sensor_service.sensor_data;

    // Loop until there's a disconnected reason
    let reason = loop {
        // Get next event
        match conn.next().await {
            // Disconnected: quit loop
            GattConnectionEvent::Disconnected { reason } => break reason,
            // GATT Event: unwrap
            GattConnectionEvent::Gatt { event } => {
                // GATT Event
                match &event {
                    // Central device reads a characteristic.
                    GattEvent::Read(event) => {
                        // Which characteristic?
                        if event.handle() == sensor_data.handle {
                            // Retrieve the value from the server and log it
                            let value = server.get(&sensor_data);
                            match server.get(&sensor_data) {
                                Ok(value) => defmt::info!("[gatt] Read Event to Sensor Data Characteristic: {:?}", value),
                                Err(e) => defmt::error!("[gatt] Get server characteristic failed: {}", e),
                            }
                        }
                    }
                    // Central device writes a characteritic.
                    GattEvent::Write(event) => {
                        // Which characteristic?
                        if event.handle() == sensor_data.handle {
                            // Data written! We are kinda callback
                            defmt::info!("[gatt] Write Event to Sensor Data Characteristic: {:?}", event.data());
                        }
                    }
                    _ => {}
                };

                // This step is also performed at drop(), but writing it explicitly is necessary
                // in order to ensure reply is sent.
                match event.accept() {
                    Ok(reply) => reply.send().await,
                    Err(e) => defmt::warn!("[gatt] error sending response: {:?}", e),
                };
            }
            _ => {} // ignore other Gatt Connection Events
        }
    };

    defmt::info!("BLE: disconnected: {:?}", reason);
    Ok(())
}


// Example task
// - Notify the connected central of a counter value every 2 seconds
// - Read the RSSI value every 2 seconds
// - Stop when the connection is closed
async fn custom_task<C: Controller, P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    stack: &Stack<'_, C, P>,
) {
    // GATT service characteristic
    let sensor_data = server.sensor_service.sensor_data;

    // The counter
    let mut tick: u8 = 0;
    loop {
        // Increment
        tick = tick.wrapping_add(1);

        // Notify everyone listening
        defmt::info!("[custom_task] notifying connection of tick {}", tick);
        if sensor_data.notify(conn, &tick).await.is_err() {
            defmt::info!("[custom_task] error notifying connection");
            break;
        };

        // Read RSSI
        if let Ok(rssi) = conn.raw().rssi(stack).await {
            defmt::info!("[custom_task] RSSI: {:?}", rssi);
        } else {
            defmt::info!("[custom_task] error getting RSSI");
            break;
        };

        // Sleep
        Timer::after_secs(2).await;
    }
}
