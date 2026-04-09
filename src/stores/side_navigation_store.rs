use std::{collections::HashMap, time::Duration};

use crate::{
    models::{AdapterInfo, Effect, IntoModelTraitAsync, ToAdapterInfosTrait, ToSlintModel},
    slint_generatedMainWindow,
    stores::StoreErr,
};
use btleplug::{
    api::{Peripheral as _, PeripheralProperties},
    platform::{Adapter, Peripheral, PeripheralId},
};
use slint_generatedMainWindow::Peripheral as PeripheralSlint;

mod adapter_info_handler;
mod peripheral_slint_handler;
mod selected_peripheral_handler;
mod slint_store_trait;

use adapter_info_handler::*;
use peripheral_slint_handler::*;
use selected_peripheral_handler::*;

pub use slint_store_trait::*;

#[derive(Default)]
pub struct SideNavigationStore {
    selected_peripheral: Effect<PeripheralSlint, SelectedPeripheralEffectHandler>,
    peripherals: Effect<Vec<PeripheralSlint>, PeripheralSlintEffectHandler>,
    adapter_infos: Effect<Vec<AdapterInfo>, AdapterInfosEffectHandler>,

    peripheral_str_id_map: HashMap<String, PeripheralId>,
    peripherals_map: HashMap<PeripheralId, Peripheral>,
    peripheral_properties_map: HashMap<PeripheralId, PeripheralProperties>,

    connected_peripheral_map: HashMap<PeripheralId, bool>,
}

impl SideNavigationStore {
    pub async fn set_adapter_infos(&mut self, adapters: &Vec<Adapter>) {
        let adapter_infos = adapters.to_adapter_infos().await;

        self.adapter_infos
            .set(adapter_infos)
            .await
            .expect("fail to set adapters");
    }

    pub async fn set_peripheral_map(&mut self, pid: PeripheralId, peripheral: Peripheral) {
        self.peripherals_map.insert(pid.clone(), peripheral.clone());

        self.peripheral_str_id_map
            .insert(pid.clone().to_string(), pid);

        let peripheral = peripheral.into_model_async().await;

        self.peripherals
            .insert(peripheral.to_slint_model())
            .await
            .expect("fail to insert peripheral");
    }

    pub fn set_peripheral_properties_map(
        &mut self,
        pid: PeripheralId,
        peripheral_properties: PeripheralProperties,
    ) {
        self.peripheral_properties_map
            .insert(pid, peripheral_properties);
    }

    pub fn set_connected_peripheral(&mut self, pid: PeripheralId, is_connected: bool) {
        self.connected_peripheral_map.insert(pid, is_connected);
    }
}

impl SideNavigationSlintStoreTrait for SideNavigationStore {
    async fn handle_on_connect(&mut self, peripheral_id: String) -> Result<(), StoreErr> {
        let pid = self
            .peripheral_str_id_map
            .get(&peripheral_id)
            .ok_or(StoreErr::Custom("cant get pid".into()))?;

        let peripheral = self
            .peripherals_map
            .get(pid)
            .ok_or(StoreErr::Custom("fail to get peripheral".into()))?;

        let is_connected = peripheral
            .is_connected()
            .await
            .map_err(|_| StoreErr::Custom("fail to check connection status".into()))?;

        if is_connected {
            return Ok(());
        }

        peripheral
            .connect_with_timeout(Duration::from_millis(5000))
            .await
            .map_err(|err| StoreErr::Custom(err.to_string()))?;

        Ok(())
    }

    async fn handle_on_disconnect(&mut self, peripheral_id: String) -> Result<(), StoreErr> {
        let pid = self
            .peripheral_str_id_map
            .get(&peripheral_id)
            .ok_or(StoreErr::Custom("cant get pid".into()))?;

        let peripheral = self
            .peripherals_map
            .get(pid)
            .ok_or(StoreErr::Custom("fail to get peripheral".into()))?;

        let is_connected = peripheral
            .is_connected()
            .await
            .map_err(|_| StoreErr::Custom("fail to check connection status".into()))?;

        if !is_connected {
            return Ok(());
        }

        peripheral
            .discover_services_with_timeout(Duration::from_millis(5000))
            .await
            .map_err(|err| StoreErr::Custom(err.to_string()))?;

        Ok(())
    }
    async fn handle_on_rescan_all(&mut self) -> Result<(), StoreErr> {
        todo!()
    }

    async fn handle_on_rescan_adapter(&mut self, adapter_id: String) -> Result<(), StoreErr> {
        todo!()
    }

    async fn handle_on_peripheral_clicked(
        &mut self,
        peripheral_id: String,
    ) -> Result<(), StoreErr> {
        todo!()
    }
}
