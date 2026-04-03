use crate::helpers::set_ui_state::set_ui_state;
use crate::{DevicesStoreSlint, MainWindow, NavigationItem};
use btleplug::platform::Adapter;
use slint::{ComponentHandle, EventLoopError, Weak};
use slint::{Image, ModelRc, VecModel};

mod initialization;

pub trait DevicesStoreTrait {
    fn set_adapters(&mut self, adapters: Vec<Adapter>);
    fn set_adapter_infos(&mut self, adapters: Vec<String>);
    fn get_adapter_infos(&mut self) -> impl Future<Output = Vec<String>>;
}

pub struct DevicesStore {
    adapters: Vec<Adapter>,
    weak_main_window: Weak<MainWindow>,
}

impl DevicesStore {
    pub fn new() -> Self {
        Self {
            adapters: [].to_vec(),
            weak_main_window: Weak::default(),
        }
    }

    pub fn set_weak_main_window(&mut self, weak_main_window: Weak<MainWindow>) {
        self.weak_main_window = weak_main_window;
    }
}

impl DevicesStoreTrait for DevicesStore {
    fn set_adapters(&mut self, adapters: Vec<Adapter>) {
        self.adapters = adapters;
    }

    async fn get_adapter_infos(&mut self) -> Vec<String> {
        todo!()
    }

    fn set_adapter_infos(&mut self, adapters: Vec<String>) {
        let result = set_ui_state(&self.weak_main_window, |main_window| {
            let device_stores = main_window.global::<DevicesStoreSlint>();

            let nav_items = ModelRc::new(VecModel::from(
                adapters
                    .into_iter()
                    .map(|item| NavigationItem {
                        text: item.into(),
                        badge: "".into(),
                        icon: Image::default(),
                        selected_icon: Image::default(),
                        show_badge: false,
                    })
                    .collect::<Vec<_>>(),
            ));
            device_stores.set_adapterInfoNavItems(nav_items);
        });

        match result {
            Ok(_) => {}
            Err(_) => println!("fail to set ui state"),
        }
    }
}

#[cfg(test)]
mod devices_store_tests {

    #[test]
    fn name() {
        todo!();
    }
}
