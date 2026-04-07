use crate::MAINWINDOW_WEAK;
use crate::helpers::set_ui_state::set_ui_state;
use crate::store_handler::SideEffectHandlerTrait;
use btleplug::platform::Adapter;

impl SideEffectHandlerTrait for Vec<Adapter> {
    type ValType = Self;

    async fn on_set(
        window_weak: &crate::MainWindow,
        val: Self::ValType,
    ) -> Result<(), super::SideEffectValError> {
        if let Some(main_window) = MAINWINDOW_WEAK.get() {
            let result = set_ui_state(main_window, move |main_window| {
                // TODO: Update
            });

            match result {
                Ok(_) => {}
                Err(_) => println!("fail to set ui state"),
            };
        }
        Ok(())
    }

    async fn on_get(
        window_weak: &crate::MainWindow,
    ) -> Result<Self::ValType, super::SideEffectValError> {
        todo!()
    }
}
