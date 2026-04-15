use crate::stores::{
    devices_store::DevicesStore, global_stores::GlobalStore,
    side_navigation_store::SideNavigationStore,
};

use std::sync::LazyLock;
use tokio::sync::Mutex;

pub mod devices_store;
pub mod global_stores;
pub mod side_navigation_store;

#[derive(Debug)]
pub enum StoreErr {
    FailToSet,
    FailToGet,
    Custom(String),
}

pub static SIDE_NAV_STORE: LazyLock<Mutex<SideNavigationStore>> =
    LazyLock::new(|| Mutex::new(SideNavigationStore::default()));

pub static DEVICES_STORE: LazyLock<Mutex<DevicesStore>> =
    LazyLock::new(|| Mutex::new(DevicesStore::default()));

pub static GLOBAL_STORE: LazyLock<Mutex<GlobalStore>> =
    LazyLock::new(|| Mutex::new(GlobalStore::default()));
