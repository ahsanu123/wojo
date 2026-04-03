use std::collections::HashMap;

use super::DevicesStore;
use crate::{InitTrait, ble::ble_manager, stores::devices_store::DevicesStoreTrait};
use btleplug::{
    api::{Central, Manager},
    platform::Peripheral,
};

impl InitTrait for DevicesStore {
    async fn init(&mut self) {
        self.adapters = ble_manager().await.adapters().await.unwrap_or([].to_vec());

        if self.adapters.is_empty() {
            return;
        }

        let mut infos = Vec::<String>::new();
        let mut peripheral_map = HashMap::<String, Vec<Peripheral>>::new();

        for adapter in self.adapters.clone() {
            let peripherals = adapter.peripherals().await.unwrap_or([].to_vec());

            if let Ok(info) = adapter.adapter_info().await {
                infos.push(info.clone());

                if !peripherals.is_empty() {
                    peripheral_map.insert(info.clone(), peripherals);
                }
            }
        }

        self.set_adapter_infos(infos);
    }
}
