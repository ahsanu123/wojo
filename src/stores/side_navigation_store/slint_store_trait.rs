use crate::stores::StoreErr;

pub trait SideNavigationSlintStoreTrait {
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
