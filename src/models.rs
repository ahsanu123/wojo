mod adapter_info;
mod adapter_to_adapter_info;
mod effect;
mod get_id_trait;
mod into_model_trait;
mod peripheral;

pub use adapter_info::*;
pub use adapter_to_adapter_info::*;
pub use effect::*;
pub use get_id_trait::*;
pub use into_model_trait::*;
pub use peripheral::*;

pub trait ToSlintModel<T> {
    fn to_slint_model(&self) -> T;
}

pub trait ToSlintModelAsync<T> {
    async fn to_slint_model_async(&self) -> T;
}
