use crate::{slint_generatedMainWindow, store_handler::SideEffectHandlerTrait};

impl SideEffectHandlerTrait for Vec<slint_generatedMainWindow::Peripheral> {
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

impl SideEffectHandlerTrait for slint_generatedMainWindow::Peripheral {
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
