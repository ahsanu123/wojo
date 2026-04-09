use slint::{ComponentHandle as _, ModelRc};

use crate::{
    SideNavigationStore,
    helpers::set_ui_state::set_ui_state,
    models::{AdapterInfo, StoreHandlerErr, StoreHandlerTrait, ToSlintModel},
};

#[derive(Default)]
pub struct AdapterInfosEffectHandler;

impl StoreHandlerTrait<Vec<AdapterInfo>> for AdapterInfosEffectHandler {
    async fn on_set(
        window_weak: &slint::Weak<crate::MainWindow>,
        value: Vec<AdapterInfo>,
    ) -> Result<(), StoreHandlerErr> {
        set_ui_state(window_weak, move |main_window| {
            let sidenav_store = main_window.global::<SideNavigationStore>();

            let vecmodel_adapter_info = value.to_slint_model();
            let modelrc = ModelRc::new(vecmodel_adapter_info);

            sidenav_store.set_adapterInfos(modelrc);
        })
        .map_err(|_| StoreHandlerErr::FailToSet)?;

        Ok(())
    }

    async fn on_get(
        _window_weak: &slint::Weak<crate::MainWindow>,
    ) -> Result<Vec<AdapterInfo>, StoreHandlerErr> {
        // NOTE: for now nothing to do
        Ok([].to_vec())
    }
}
