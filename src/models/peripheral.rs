use crate::{
    models::{GetIdTrait, IntoModelTrait, IntoModelTraitAsync, ToSlintModel, ToSlintModelAsync},
    slint_generatedMainWindow,
};
use btleplug::{api::Peripheral as _, platform::Peripheral as BtleplugPeripheral};
use slint::{SharedString, VecModel};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Peripheral {
    peripheral_id: String,
    address: String,
    localname: String,
    is_connected: bool,
    rssi: i32,
}

impl GetIdTrait for Peripheral {
    type IdType = String;

    fn get_id(&self) -> Self::IdType {
        self.peripheral_id.clone()
    }
}

impl GetIdTrait for slint_generatedMainWindow::Peripheral {
    type IdType = SharedString;

    fn get_id(&self) -> Self::IdType {
        self.peripheralId.clone()
    }
}

impl IntoModelTrait<Peripheral> for btleplug::platform::Peripheral {
    fn into_model(&self) -> Peripheral {
        Peripheral {
            peripheral_id: todo!(),
            address: todo!(),
            localname: todo!(),
            is_connected: todo!(),
            rssi: todo!(),
        }
    }
}

impl ToSlintModel<slint_generatedMainWindow::Peripheral> for Peripheral {
    fn to_slint_model(&self) -> slint_generatedMainWindow::Peripheral {
        todo!()
    }
}

impl ToSlintModel<VecModel<slint_generatedMainWindow::Peripheral>> for Vec<Peripheral> {
    fn to_slint_model(&self) -> VecModel<slint_generatedMainWindow::Peripheral> {
        self.iter()
            .map(|item| slint_generatedMainWindow::Peripheral {
                address: item.address.clone().into(),
                isConnected: item.is_connected,
                localname: item.localname.clone().into(),
                peripheralId: item.peripheral_id.clone().into(),
                rssi: item.rssi,
            })
            .collect()
    }
}

impl IntoModelTraitAsync<Peripheral> for BtleplugPeripheral {
    async fn into_model_async(&self) -> Peripheral {
        let properties = self
            .properties()
            .await
            .expect("fail to get peripheral properties");

        let is_connected = self.is_connected().await.expect("fail to get is connected");

        let address = self.address().to_string();

        let localname = properties
            .clone()
            .and_then(|p| p.local_name)
            .unwrap_or(address.clone());

        let rssi = properties.and_then(|p| p.rssi).unwrap_or(0);

        Peripheral {
            peripheral_id: self.id().to_string(),
            address,
            localname,
            is_connected,
            rssi: rssi as i32,
        }
    }
}
