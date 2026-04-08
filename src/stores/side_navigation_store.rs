use crate::{
    models::{AdapterInfo, Effect, ToAdapterInfosTrait},
    slint_generatedMainWindow,
    stores::StoreErr,
};
use btleplug::platform::Adapter;
use slint_generatedMainWindow::Peripheral as PeripheralSlint;

mod adapter_info_handler;
mod peripheral_slint_handler;
mod selected_peripheral_handler;
mod store_trait;

use adapter_info_handler::*;
use peripheral_slint_handler::*;
use selected_peripheral_handler::*;

pub use store_trait::*;

#[derive(Default)]
pub struct SideNavigationStore {
    selected_peripheral: Effect<PeripheralSlint, SelectedPeripheralEffectHandler>,
    peripherals: Effect<Vec<PeripheralSlint>, PeripheralSlintEffectHandler>,
    adapter_infos: Effect<Vec<AdapterInfo>, AdapterInfosEffectHandler>,
}

impl SideNavigationStore {
    pub async fn set_adapters(&mut self, adapters: &Vec<Adapter>) {
        let adapter_infos = adapters.to_adapter_infos().await;

        self.adapter_infos
            .set(adapter_infos)
            .await
            .expect("fail to set adapters");
    }
}

impl SideNavigationStoreTrait for SideNavigationStore {
    async fn handle_on_connect(&mut self, peripheral_id: String) -> Result<(), StoreErr> {
        todo!()
    }

    async fn handle_on_disconnect(&mut self, peripheral_id: String) -> Result<(), StoreErr> {
        todo!()
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
