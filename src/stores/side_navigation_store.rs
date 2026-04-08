use crate::{
    models::{AdapterInfo, Effect},
    slint_generatedMainWindow,
    stores::StoreErr,
};
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
    pub selected_peripheral: Effect<PeripheralSlint, SelectedPeripheralEffectHandler>,
    pub peripherals: Effect<Vec<PeripheralSlint>, PeripheralSlintEffectHandler>,
    pub adapters: Effect<Vec<AdapterInfo>, AdaptersEffectHandler>,
}

impl SideNavigationStore {
    pub fn set_adapters(&mut self, adapters: Effect<Vec<AdapterInfo>, AdaptersEffectHandler>) {
        self.adapters = adapters;
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
