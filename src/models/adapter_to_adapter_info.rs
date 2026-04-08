use crate::models::AdapterInfo;
use btleplug::{api::Central, platform::Adapter};

pub trait ToAdapterInfosTrait {
    fn to_adapter_infos(&self) -> impl Future<Output = Vec<AdapterInfo>>;
}

pub trait ToAdapterInfoTrait {
    fn to_adapter_info(&self) -> impl Future<Output = Option<AdapterInfo>>;
}

impl ToAdapterInfosTrait for Vec<Adapter> {
    async fn to_adapter_infos(&self) -> Vec<AdapterInfo> {
        let mut adapter_infos: Vec<AdapterInfo> = Vec::new();

        for adapter in self {
            if let Ok(info) = adapter.adapter_info().await {
                if let Some(short_name) = info
                    .split(' ')
                    .map(|word| word.into())
                    .collect::<Vec<String>>()
                    .first()
                {
                    adapter_infos.push(AdapterInfo {
                        name: short_name.clone(),
                        long_name: info,
                    });
                } else {
                    adapter_infos.push(AdapterInfo {
                        name: info.clone(),
                        long_name: info.clone(),
                    });
                }
            }
        }

        adapter_infos
    }
}

impl ToAdapterInfoTrait for Adapter {
    async fn to_adapter_info(&self) -> Option<AdapterInfo> {
        let result = self.adapter_info().await;

        match result {
            Ok(info) => {
                if let Some(short_name) = info
                    .split(' ')
                    .map(|word| word.into())
                    .collect::<Vec<String>>()
                    .first()
                {
                    let adapter_info = AdapterInfo {
                        name: short_name.clone(),
                        long_name: info,
                    };

                    Some(adapter_info)
                } else {
                    let adapter_info = AdapterInfo {
                        name: info.clone(),
                        long_name: info.clone(),
                    };

                    Some(adapter_info)
                }
            }
            Err(_) => None,
        }
    }
}
