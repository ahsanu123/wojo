pub trait IntoModelTrait<T> {
    fn into_model(&self) -> T;
}

pub trait IntoModelTraitAsync<T> {
    async fn into_model_async(&self) -> T;
}
