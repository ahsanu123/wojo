use btleplug::{
    api::{PeripheralProperties, Service},
    platform::PeripheralId,
};
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub struct Peripheral {
    pub id: PeripheralId,
    pub is_connected: bool,
    pub rssi: i16,
    pub properties: PeripheralProperties,
    pub services: BTreeSet<Service>,
}

impl Peripheral {
    pub fn new(id: PeripheralId) -> Self {
        Self {
            id,
            is_connected: false,
            rssi: 0,
            properties: PeripheralProperties::default(),
            services: BTreeSet::new(),
        }
    }
}
