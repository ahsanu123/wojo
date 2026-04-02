use super::DevicesStore;
use crate::{InitTrait, ble::ble_manager};
use btleplug::api::{Central, Manager};

impl InitTrait for DevicesStore {
    async fn init(&mut self) {
        self.adapters = ble_manager().await.adapters().await.unwrap_or([].to_vec());

        if self.adapters.is_empty() {
            return;
        }

        for adapter in self.adapters.clone() {
            if let Ok(info) = adapter.adapter_info().await
                && let Ok(state) = adapter.adapter_state().await
            {
                self.adapter_infos.push(info.clone());
                self.adapter_central_states.insert(info, state);
            }
        }
    }
}
