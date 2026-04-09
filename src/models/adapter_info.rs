use crate::{models::ToSlintModel, slint_generatedMainWindow};
use slint::VecModel;

#[derive(Default, Clone)]
pub struct AdapterInfo {
    pub name: String,
    pub long_name: String,
}

impl ToSlintModel<VecModel<slint_generatedMainWindow::AdapterInfo>> for Vec<AdapterInfo> {
    fn to_slint_model(&self) -> VecModel<slint_generatedMainWindow::AdapterInfo> {
        self.iter()
            .map(|adapter_info| slint_generatedMainWindow::AdapterInfo {
                name: adapter_info.name.clone().into(),
                long_name: adapter_info.long_name.clone().into(),
            })
            .collect()
    }
}
