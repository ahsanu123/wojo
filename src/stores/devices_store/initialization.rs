use super::DevicesStore;
use crate::{InitTrait, ble::ble_manager, stores::devices_store::DevicesStoreTrait};
use btleplug::api::{Central, Manager};

impl InitTrait for DevicesStore {
    async fn init(&mut self) {
        self.adapters = ble_manager().await.adapters().await.unwrap_or([].to_vec());

        if self.adapters.is_empty() {
            return;
        }

        let mut infos = Vec::<String>::new();

        for adapter in self.adapters.clone() {
            if let Ok(info) = adapter.adapter_info().await {
                infos.push(info.clone());
            }
        }

        self.set_adapter_infos(infos);
    }
}
