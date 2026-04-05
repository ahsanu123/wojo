use slint::Weak;

use crate::MainWindow;

use super::SideEffectValError;

pub trait SideEffectHandlerTrait {
    type ValType;

    fn on_set(
        window_weak: &MainWindow,
        val: Self::ValType,
    ) -> impl Future<Output = Result<(), SideEffectValError>>;

    fn on_get(
        window_weak: &MainWindow,
    ) -> impl Future<Output = Result<Self::ValType, SideEffectValError>>;
}
