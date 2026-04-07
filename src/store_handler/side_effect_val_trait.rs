use crate::MAINWINDOW_WEAK;

use super::SideEffectHandlerTrait;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum SideEffectValError {
    FailToSet,
    FailToGet,
}

#[derive(Default)]
pub struct SideEffectVal<T>
where
    T: SideEffectHandlerTrait,
{
    value: T,
}

impl<T> SideEffectVal<T>
where
    T: SideEffectHandlerTrait,
{
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> SideEffectVal<T>
where
    T: Clone + SideEffectHandlerTrait<ValType = T>,
{
    pub async fn set(&mut self, val: T) -> Result<(), SideEffectValError> {
        self.value = val.clone();

        if let Some(window_weak) = MAINWINDOW_WEAK.get()
            && let Some(window) = window_weak.upgrade()
        {
            T::on_set(&window, val).await?;
        };

        Ok(())
    }

    pub async fn get(&mut self) -> Result<T, SideEffectValError> {
        if let Some(window_weak) = MAINWINDOW_WEAK.get()
            && let Some(window) = window_weak.upgrade()
        {
            T::on_get(&window).await?;
        };
        Ok(self.value.clone())
    }
}

#[cfg(test)]
mod side_effect_val_trait_test {

    use super::*;
    use crate::models::Item;

    fn new_side_val<T>(value: T) -> SideEffectVal<T>
    where
        T: SideEffectHandlerTrait,
    {
        SideEffectVal { value }
    }

    #[tokio::test]
    async fn test_on_item() {
        let mut sv = new_side_val(Item {
            val: String::from("hellow"),
        });

        sv.set(Item {
            val: String::from("updated value"),
        })
        .await
        .unwrap();

        let mut svs = new_side_val(
            [
                Item {
                    val: String::from("hellow"),
                },
                Item {
                    val: String::from("hellow"),
                },
            ]
            .to_vec(),
        );
        svs.set([].to_vec()).await.unwrap();
    }
}
