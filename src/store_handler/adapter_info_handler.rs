use crate::{models::AdapterInfo, store_handler::SideEffectHandlerTrait};

impl SideEffectHandlerTrait for Vec<AdapterInfo> {
    type ValType = Self;

    async fn on_set(
        window_weak: &crate::MainWindow,
        val: Self::ValType,
    ) -> Result<(), super::SideEffectValError> {
        todo!()
    }

    async fn on_get(
        window_weak: &crate::MainWindow,
    ) -> Result<Self::ValType, super::SideEffectValError> {
        todo!()
    }
}
