use btleplug::{api::CentralState, platform::Adapter};
use std::collections::HashMap;

use crate::{MsgTypeAndVal, TX};

mod initialization;

pub trait DevicesStoreTrait {
    fn set_adapters(&mut self, adapters: Vec<Adapter>);
    fn get_adapter_infos(&mut self) -> impl Future<Output = Vec<String>>;
}

pub struct DevicesStore {
    pub adapters: Vec<Adapter>,
    pub adapter_infos: Vec<String>,
    pub adapter_central_states: HashMap<String, CentralState>,
}

impl DevicesStore {
    pub fn new() -> Self {
        Self {
            adapters: [].to_vec(),
            adapter_infos: [].to_vec(),
            adapter_central_states: HashMap::new(),
        }
    }
}

impl DevicesStoreTrait for DevicesStore {
    fn set_adapters(&mut self, adapters: Vec<Adapter>) {
        self.adapters = adapters;
    }

    async fn get_adapter_infos(&mut self) -> Vec<String> {
        if let Some(tx) = TX.get() {
            tx.send(MsgTypeAndVal::SetAdapters(self.adapter_infos.clone()))
                .await
                .expect("fail to send adapter infos");
        }

        self.adapter_infos.clone()
    }
}

#[cfg(test)]
mod devices_store_tests {

    #[test]
    fn name() {
        todo!();
    }
}
