use crate::{
    models::{StoreHandlerErr, StoreHandlerTrait},
    slint_generatedMainWindow,
};
use slint::Weak;
use slint_generatedMainWindow::Peripheral as PeripheralSlint;

#[derive(Default)]
pub struct SelectedPeripheralEffectHandler;

impl StoreHandlerTrait<PeripheralSlint> for SelectedPeripheralEffectHandler {
    async fn on_set(
        window_weak: &Weak<crate::MainWindow>,
        value: PeripheralSlint,
    ) -> Result<(), StoreHandlerErr> {
        todo!()
    }

    async fn on_get(
        window_weak: &Weak<crate::MainWindow>,
    ) -> Result<PeripheralSlint, StoreHandlerErr> {
        todo!()
    }
}
