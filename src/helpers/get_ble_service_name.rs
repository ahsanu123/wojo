use btleplug::api::bleuuid::BleUuid;
use btuuid::{
    BluetoothUuid16,
    service::{BATTERY, HUMAN_INTERFACE_DEVICE},
};
use uuid::Uuid;

pub fn get_ble_service_name(uuid: Uuid) -> String {
    if let Some(usixteen) = uuid.to_ble_u16() {
        let ble_uuid = BluetoothUuid16::new(usixteen);

        match ble_uuid {
            BATTERY => "Battery Service",
            HUMAN_INTERFACE_DEVICE => "Human Machine Interface",
            _ => "unknown service",
        }
        .into()
    } else {
        "unknown service".into()
    }
}
