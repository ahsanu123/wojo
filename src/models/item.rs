use crate::store_handler::{SideEffectHandlerTrait, SideEffectValError};

#[derive(Clone)]
pub struct Item {
    pub val: String,
}

impl SideEffectHandlerTrait for Item {
    type ValType = Self;

    async fn on_set(
        window_weak: &crate::MainWindow,
        val: Self::ValType,
    ) -> Result<(), SideEffectValError> {
        todo!()
    }

    async fn on_get(window_weak: &crate::MainWindow) -> Result<Self::ValType, SideEffectValError> {
        todo!()
    }
}

impl SideEffectHandlerTrait for Vec<Item> {
    type ValType = Self;

    async fn on_set(
        window_weak: &crate::MainWindow,
        val: Self::ValType,
    ) -> Result<(), SideEffectValError> {
        todo!()
    }

    async fn on_get(window_weak: &crate::MainWindow) -> Result<Self::ValType, SideEffectValError> {
        todo!()
    }
}
