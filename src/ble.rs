use btleplug::{
    api::Manager as _,
    platform::{Adapter, Manager},
};
use tokio::sync::OnceCell;

pub mod characteristic;
pub mod services;

static BLE_MANAGER: OnceCell<Manager> = OnceCell::const_new();

pub async fn ble_manager() -> &'static Manager {
    BLE_MANAGER
        .get_or_init(|| async { Manager::new().await.expect("cant init manager") })
        .await
}

pub async fn list_adapters() -> Vec<Adapter> {
    let manager = ble_manager().await;
    manager.adapters().await.expect("cant get list adapter!!")
}

#[cfg(test)]
mod test {
    use btleplug::api::Central;

    use super::*;

    #[tokio::test]
    async fn test_list_ble_adapters() {
        let adapters = list_adapters().await;
        println!("adapters: {adapters:?}");

        for adapter in adapters {
            let info = adapter.adapter_info().await;
            println!("-------------------------------------------");
            println!("adapter-info: {info:#?}");
        }
    }
}
