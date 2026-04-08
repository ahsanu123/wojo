use crate::{
    MAINWINDOW_WEAK, SideNavigationStore, helpers::set_ui_state::set_ui_state, models::AdapterInfo,
};
use slint::{ComponentHandle as _, ModelRc, SharedString, VecModel};
use std::marker::PhantomData;

pub enum StoreHandlerErr {
    FailToSet,
    FailToGet,
}

pub trait StoreHandlerTrait<T> {
    fn on_set(
        window_weak: &slint::Weak<crate::MainWindow>,
        value: T,
    ) -> impl Future<Output = Result<(), StoreHandlerErr>>;

    fn on_get(
        window_weak: &slint::Weak<crate::MainWindow>,
    ) -> impl Future<Output = Result<T, StoreHandlerErr>>;
}

#[derive(Default)]
pub struct Effect<T, H>
where
    T: Clone,
    H: StoreHandlerTrait<T>,
{
    value: T,

    _handler: PhantomData<H>,
}

impl<T, H> Effect<T, H>
where
    T: Clone,
    H: StoreHandlerTrait<T>,
{
    pub async fn set(&mut self, value: T) -> Result<(), StoreHandlerErr> {
        self.value = value.clone();

        if let Some(window_weak) = MAINWINDOW_WEAK.get() {
            H::on_set(window_weak, value).await?;
        };

        Ok(())
    }

    pub async fn get(&mut self) -> Result<T, StoreHandlerErr> {
        // TODO: think better in here
        if let Some(window_weak) = MAINWINDOW_WEAK.get() {
            H::on_get(window_weak).await?;
        };

        Ok(self.value.clone())
    }
}
