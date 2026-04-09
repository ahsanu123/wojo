use crate::{
    ExtendedCentralEvent,
    ble::ble_manager,
    stores::{DEVICES_STORE, SIDE_NAV_STORE, devices_store::DevicesStoreTrait},
};
use btleplug::api::{Central, CentralEvent, Manager, Peripheral};
use tokio::sync::mpsc::Receiver;

pub async fn background_task(mut rx: Receiver<ExtendedCentralEvent>) {
    while let Some(event) = rx.recv().await {
        match event {
            ExtendedCentralEvent::Exit => {
                break;
            }
            ExtendedCentralEvent::Base(CentralEvent::DeviceDiscovered(peripheral_id)) => {
                let adapters = ble_manager().await.adapters().await.unwrap();
                let central = adapters.first().unwrap();

                if let Ok(peripheral) = central.peripheral(&peripheral_id).await {
                    let mut store = SIDE_NAV_STORE.lock().await;

                    store.set_peripheral_map(peripheral_id.clone(), peripheral.clone());

                    if let Ok(maybe_properties) = &peripheral.properties().await
                        && let Some(properties) = maybe_properties
                    {
                        store.set_peripheral_properties_map(peripheral_id, properties.clone());
                    }
                }
            }
            ExtendedCentralEvent::Base(CentralEvent::DeviceConnected(peripheral_id)) => {
                let mut store = SIDE_NAV_STORE.lock().await;

                store.set_connected_peripheral(peripheral_id, true);
            }
            ExtendedCentralEvent::Base(CentralEvent::DeviceDisconnected(peripheral_id)) => {
                let mut store = SIDE_NAV_STORE.lock().await;

                store.set_connected_peripheral(peripheral_id, false);
            }
            ExtendedCentralEvent::Base(CentralEvent::RssiUpdate { id, rssi }) => {
                let mut store = DEVICES_STORE.lock().await;
                store.set_peripheral_rssi(id, rssi).await;
            }
            ExtendedCentralEvent::Base(CentralEvent::DeviceUpdated(peripheral_id)) => {
                println!("DeviceUpdated")
            }
            ExtendedCentralEvent::Base(CentralEvent::DeviceServicesModified(peripheral_id)) => {
                println!("DeviceServicesModified")
            }
            ExtendedCentralEvent::Base(CentralEvent::ServiceDataAdvertisement {
                id,
                service_data,
            }) => {
                println!("ServiceDataAdvertisement")
            }
            ExtendedCentralEvent::Base(CentralEvent::ServicesAdvertisement { id, services }) => {
                println!("ServicesAdvertisement")
            }
            ExtendedCentralEvent::Base(CentralEvent::StateUpdate(central_state)) => {
                println!("StateUpdate")
            }
            ExtendedCentralEvent::Base(CentralEvent::ManufacturerDataAdvertisement {
                id,
                manufacturer_data,
            }) => println!("ManufacturerDataAdvertisement"),
        }
    }
}
