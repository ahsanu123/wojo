use btleplug::api::Central as _;
use btleplug::{api::Manager as _, platform::Manager};
use tokio::sync::OnceCell;

pub mod characteristic;
pub mod services;

static BLE_MANAGER: OnceCell<Manager> = OnceCell::const_new();

pub async fn ble_manager() -> &'static Manager {
    BLE_MANAGER
        .get_or_init(|| async { Manager::new().await.expect("cant init manager") })
        .await
}

#[cfg(test)]
mod ble_tests {

    use super::*;

    #[tokio::test]
    async fn test_list_ble_adapters() {
        let adapters = ble_manager()
            .await
            .adapters()
            .await
            .expect("fail to list adapters");

        println!("adapters: {adapters:?}");

        for adapter in adapters {
            let info = adapter.adapter_info().await;
            println!("-------------------------------------------");
            println!("adapter-info: {info:#?}");
        }
    }
}
