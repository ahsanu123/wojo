use crate::{
    models::AdapterInfo, slint_generatedMainWindow, store_handler::SideEffectVal, stores::StoreErr,
};

pub trait SideNavigationStoreTrait {
    fn handle_on_rescan_all(&mut self) -> impl Future<Output = Result<(), StoreErr>>;

    fn handle_on_connect(
        &mut self,
        peripheral_id: String,
    ) -> impl Future<Output = Result<(), StoreErr>>;

    fn handle_on_disconnect(
        &mut self,
        peripheral_id: String,
    ) -> impl Future<Output = Result<(), StoreErr>>;

    fn handle_on_rescan_adapter(
        &mut self,
        adapter_id: String,
    ) -> impl Future<Output = Result<(), StoreErr>>;

    fn handle_on_peripheral_clicked(
        &mut self,
        peripheral_id: String,
    ) -> impl Future<Output = Result<(), StoreErr>>;
}

#[derive(Default)]
pub struct SideNavigationStore {
    pub selected_peripheral: SideEffectVal<slint_generatedMainWindow::Peripheral>,
    pub peripherals: SideEffectVal<Vec<slint_generatedMainWindow::Peripheral>>,
    pub adapters: SideEffectVal<Vec<AdapterInfo>>,
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
