use crate::{
    MainWindow,
    models::{StoreHandlerErr, StoreHandlerTrait},
    slint_generatedMainWindow::Peripheral,
};
use slint::Weak;

#[derive(Default)]
pub struct PeripheralSlintEffectHandler;

impl StoreHandlerTrait<Vec<Peripheral>> for PeripheralSlintEffectHandler {
    fn on_set(
        window_weak: &Weak<MainWindow>,
        value: Vec<Peripheral>,
    ) -> Result<(), StoreHandlerErr> {
        todo!()
    }

    fn on_get(window_weak: &Weak<crate::MainWindow>) -> Result<Vec<Peripheral>, StoreHandlerErr> {
        todo!()
    }
}
