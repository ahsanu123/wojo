use crate::{
    MainWindow,
    models::{StoreHandlerErr, StoreHandlerTrait},
    slint_generatedMainWindow,
};
use slint::Weak;
use slint_generatedMainWindow::Peripheral as PeripheralSlint;

#[derive(Default)]
pub struct SelectedPeripheralEffectHandler;

impl StoreHandlerTrait<PeripheralSlint> for SelectedPeripheralEffectHandler {
    fn on_set(
        window_weak: &Weak<MainWindow>,
        value: PeripheralSlint,
    ) -> Result<(), StoreHandlerErr> {
        todo!()
    }

    fn on_get(window_weak: &Weak<MainWindow>) -> Result<PeripheralSlint, StoreHandlerErr> {
        todo!()
    }
}
