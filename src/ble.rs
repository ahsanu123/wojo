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

    use std::time::Duration;

    use btleplug::api::{Peripheral, ScanFilter, bleuuid::BleUuid};
    use tokio::time;
    use uuid::Uuid;

    use crate::helpers::get_ble_service_name::get_ble_service_name;

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

            let peripherals = adapter.peripherals().await.expect("cant get peripherals");
            adapter
                .start_scan(ScanFilter::default())
                .await
                .expect("Can't scan BLE adapter for connected devices...");
            time::sleep(Duration::from_secs(10)).await;
            println!("{peripherals:#?}");

            for peripheral in peripherals {
                let properties = peripheral
                    .properties()
                    .await
                    .expect("fail to get peripheral properties");

                println!("-------------------------------------------");
                println!("{properties:#?}");

                println!("-------------------------------------------");
                println!("available services");

                if let Some(props) = properties {
                    for service in props.services {
                        let service_name = get_ble_service_name(service);
                        println!("service name -> {service_name}");
                    }
                }
            }
        }
    }
}
