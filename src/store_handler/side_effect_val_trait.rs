use crate::MAINWINDOW_WEAK;

use super::SideEffectHandlerTrait;
use std::marker::PhantomData;

pub enum SideEffectValError {
    FailToSet,
    FailToGet,
}

pub struct SideEffectVal<T, H>
where
    H: SideEffectHandlerTrait,
{
    value: T,

    _handler: PhantomData<H>,
}

fn new_side_val<T>(value: T) -> SideEffectVal<T, T>
where
    T: SideEffectHandlerTrait,
{
    SideEffectVal {
        value,
        _handler: PhantomData,
    }
}

impl<T, H> SideEffectVal<T, H>
where
    H: SideEffectHandlerTrait<ValType = T>,
    T: Clone + Copy,
{
    pub async fn set(&mut self, val: T) -> Result<(), SideEffectValError> {
        self.value = val;

        if let Some(window_weak) = MAINWINDOW_WEAK.get()
            && let Some(window) = window_weak.upgrade()
        {
            H::on_set(&window, val).await?;
        };

        Ok(())
    }
    pub async fn get(&mut self) -> Result<T, SideEffectValError> {
        if let Some(window_weak) = MAINWINDOW_WEAK.get()
            && let Some(window) = window_weak.upgrade()
        {
            H::on_get(&window).await?;
        };
        Ok(self.value)
    }
}

#[cfg(test)]
mod side_effect_val_trait_test {

    use crate::models::Item;

    use super::*;
    #[test]
    fn test_on_item() {
        let _sv = new_side_val(Item {
            val: String::from("hellow"),
        });
    }
}
