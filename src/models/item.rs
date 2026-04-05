use crate::store_handler::{SideEffectHandlerTrait, SideEffectValError};

pub struct Item {
    pub val: String,
}

impl SideEffectHandlerTrait for Item {
    type ValType = Item;

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
