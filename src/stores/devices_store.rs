use std::collections::HashMap;

use crate::helpers::set_ui_state::set_ui_state;
use crate::{DevicesStoreSlint, MainWindow, NavigationItem};
use btleplug::platform::{Adapter, Peripheral};
use slint::{ComponentHandle, Weak};
use slint::{Image, ModelRc, VecModel};

mod initialization;

pub trait DevicesStoreTrait {
    fn set_adapters(&mut self, adapters: Vec<Adapter>);
    fn set_adapter_infos(&mut self, adapters: Vec<String>);

    fn set_adapter_peripherals_map(
        &mut self,
        adapter_peripheral_map: HashMap<String, Vec<Peripheral>>,
    );
}

pub struct DevicesStore {
    adapters: Vec<Adapter>,
    adapter_peripherals_map: HashMap<String, Vec<Peripheral>>,
    weak_main_window: Weak<MainWindow>,
}

impl DevicesStore {
    pub fn default() -> Self {
        Self {
            adapters: [].to_vec(),
            weak_main_window: Weak::default(),
            adapter_peripherals_map: HashMap::new(),
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

    fn set_adapter_infos(&mut self, adapters: Vec<String>) {
        let result = set_ui_state(&self.weak_main_window, |main_window| {
            let device_stores = main_window.global::<DevicesStoreSlint>();

            let nav_items = ModelRc::new(VecModel::from(
                adapters
                    .into_iter()
                    .map(|item| {
                        let adapter_name: Vec<String> =
                            item.split(' ').map(|s| s.to_string()).collect();

                        NavigationItem {
                            text: adapter_name.first().unwrap().into(),
                            badge: "O".into(),
                            icon: Image::load_from_svg_data(include_bytes!(
                                "../../ui/slint-logo.svg"
                            ))
                            .unwrap(),
                            selected_icon: Image::default(),
                            show_badge: false,
                        }
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

    fn set_adapter_peripherals_map(
        &mut self,
        adapter_peripheral_map: HashMap<String, Vec<Peripheral>>,
    ) {
        todo!()
    }
}

#[cfg(test)]
mod devices_store_tests {

    #[test]
    fn name() {
        todo!();
    }
}
