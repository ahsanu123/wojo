use crate::{
    models::{StoreHandlerErr, StoreHandlerTrait},
    slint_generatedMainWindow,
};
use slint::Weak;

#[derive(Default)]
pub struct PeripheralSlintEffectHandler;

impl StoreHandlerTrait<Vec<slint_generatedMainWindow::Peripheral>>
    for PeripheralSlintEffectHandler
{
    async fn on_set(
        window_weak: &Weak<crate::MainWindow>,
        value: Vec<slint_generatedMainWindow::Peripheral>,
    ) -> Result<(), StoreHandlerErr> {
        todo!()
    }

    async fn on_get(
        window_weak: &Weak<crate::MainWindow>,
    ) -> Result<Vec<slint_generatedMainWindow::Peripheral>, StoreHandlerErr> {
        todo!()
    }
}
