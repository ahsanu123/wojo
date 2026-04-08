use crate::slint_generatedMainWindow;
use slint::VecModel;

#[derive(Default, Clone)]
pub struct AdapterInfo {
    pub name: String,
    pub long_name: String,
}

pub trait AdapterInfoToSlintAdapterInfoTrait {
    fn to_vecmodel_adapter_info(&self) -> VecModel<slint_generatedMainWindow::AdapterInfo>;
}

impl AdapterInfoToSlintAdapterInfoTrait for Vec<AdapterInfo> {
    fn to_vecmodel_adapter_info(&self) -> VecModel<slint_generatedMainWindow::AdapterInfo> {
        self.iter()
            .map(|adapter_info| slint_generatedMainWindow::AdapterInfo {
                name: adapter_info.name.clone().into(),
                long_name: adapter_info.long_name.clone().into(),
            })
            .collect()
    }
}
