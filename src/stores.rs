use crate::stores::{devices_store::DevicesStore, global_stores::GlobalStore};

use std::sync::LazyLock;
use tokio::sync::Mutex;

pub mod devices_store;
pub mod global_stores;

pub static DEVICES_STORE: LazyLock<Mutex<DevicesStore>> =
    LazyLock::new(|| Mutex::new(DevicesStore::new()));

pub static GLOBAL_STORE: LazyLock<Mutex<GlobalStore>> =
    LazyLock::new(|| Mutex::new(GlobalStore::new()));
