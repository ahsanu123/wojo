use crate::{models, slint_generatedMainWindow};
use btleplug::api::{Peripheral as _, PeripheralProperties};
use btleplug::platform::{Adapter, Peripheral, PeripheralId};
use slint::ToSharedString;
use std::collections::HashMap;

pub trait DevicesStoreTrait {
    fn set_adapters(&mut self, adapters: Vec<Adapter>) -> impl Future<Output = ()>;

    fn set_adapter_infos(&mut self, adapters: Vec<String>) -> impl Future<Output = ()>;

    fn set_peripherals_map(
        &mut self,
        id: PeripheralId,
        peripheral: &Peripheral,
    ) -> impl Future<Output = ()>;

    fn set_peripheral_properties(
        &mut self,
        id: PeripheralId,
        properties: &PeripheralProperties,
    ) -> impl Future<Output = ()>;

    fn set_peripheral_rssi(&mut self, id: PeripheralId, rssi: i16) -> impl Future<Output = ()>;

    fn set_connected_peripheral(
        &mut self,
        id: PeripheralId,
        is_connected: bool,
    ) -> impl Future<Output = ()>;

    fn set_ui_state(&mut self) -> impl Future<Output = ()>;

    fn connect(&mut self, id: String) -> impl Future<Output = ()>;
}

#[derive(Default)]
pub struct DevicesStore {
    // peripheral_str_id_map: HashMap<String, PeripheralId>,
    // adapters: SideEffectVal<Vec<Adapter>>,
    // peripherals_model_map: HashMap<PeripheralId, models::Peripheral>,
    // peripherals_map: HashMap<PeripheralId, Peripheral>,
}

impl DevicesStoreTrait for DevicesStore {
    async fn set_adapters(&mut self, adapters: Vec<Adapter>) {
        todo!()
        // self.adapters.set(adapters).await.expect("fail to set");
    }

    async fn set_adapter_infos(&mut self, adapters: Vec<String>) {
        todo!()
    }

    async fn set_peripherals_map(&mut self, id: PeripheralId, peripheral: &Peripheral) {
        todo!()
        // let services = peripheral.services();
        //
        // self.peripherals_map.insert(id.clone(), peripheral.clone());
        //
        // self.peripherals_model_map
        //     .insert(id.clone(), models::Peripheral::new(id.clone()));
        //
        // self.peripheral_str_id_map
        //     .insert(format!("{:?}", id.clone()), id.clone());
        //
        // if let Some(peripheral) = self.peripherals_model_map.get_mut(&id) {
        //     peripheral.services = services;
        // }
        //
        // self.set_ui_state().await;
    }

    async fn set_connected_peripheral(&mut self, id: PeripheralId, is_connected: bool) {
        todo!()
        // if let Some(peripheral) = self.peripherals_model_map.get_mut(&id) {
        //     peripheral.is_connected = is_connected;
        // }
        // self.set_ui_state().await;
    }

    async fn set_peripheral_properties(
        &mut self,
        id: PeripheralId,
        properties: &PeripheralProperties,
    ) {
        todo!()
        // if let Some(peripheral) = self.peripherals_model_map.get_mut(&id) {
        //     peripheral.properties = properties.clone();
        // }
        // self.set_ui_state().await;
    }

    async fn set_peripheral_rssi(&mut self, id: PeripheralId, rssi: i16) {
        todo!()
        // if let Some(peripheral) = self.peripherals_model_map.get_mut(&id) {
        //     peripheral.rssi = rssi;
        // }
        // self.set_ui_state().await;
    }

    async fn set_ui_state(&mut self) {
        todo!()
        // let ui_peripheral = self
        //     .peripherals_model_map
        //     .iter()
        //     .map(|(id, peripheral)| slint_generatedMainWindow::Peripheral {
        //         peripheralId: format!("{:?}", id.clone()).into(),
        //         address: peripheral.properties.address.to_shared_string(),
        //         isConnected: peripheral.is_connected,
        //         localname: peripheral
        //             .properties
        //             .local_name
        //             .clone()
        //             .unwrap_or("empty".into())
        //             .into(),
        //         rssi: peripheral.properties.rssi.unwrap_or(0).into(),
        //     })
        //     .collect::<Vec<_>>();
    }

    async fn connect(&mut self, id: String) {
        todo!()
        // if let Some(p_id) = self.peripheral_str_id_map.get(&id)
        //     && let Some(peripheral) = self.peripherals_map.get_mut(p_id)
        // {
        //     peripheral.connect().await.unwrap();
        // }
    }
}

#[cfg(test)]
mod devices_store_tests {

    #[test]
    fn name() {
        todo!();
    }
}
