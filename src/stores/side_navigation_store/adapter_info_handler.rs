use slint::{ComponentHandle as _, ModelRc, SharedString, VecModel};

use crate::{
    SideNavigationStore,
    helpers::set_ui_state::set_ui_state,
    models::{AdapterInfo, StoreHandlerErr, StoreHandlerTrait},
};

#[derive(Default)]
pub struct AdaptersEffectHandler;

impl StoreHandlerTrait<Vec<AdapterInfo>> for AdaptersEffectHandler {
    async fn on_set(
        window_weak: &slint::Weak<crate::MainWindow>,
        value: Vec<AdapterInfo>,
    ) -> Result<(), StoreHandlerErr> {
        let adapters: Vec<String> = value.iter().map(|item| item.name.clone()).collect();

        set_ui_state(window_weak, move |main_window| {
            let sidenav_store = main_window.global::<SideNavigationStore>();

            let vecmodel_adapters: VecModel<SharedString> =
                adapters.iter().map(|item| item.into()).collect();

            let modelrc = ModelRc::new(vecmodel_adapters);
            sidenav_store.set_adapters(modelrc);
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
