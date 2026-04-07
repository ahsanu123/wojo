use crate::{
    stores::{SIDE_NAV_STORE, side_navigation_store::SideNavigationStoreTrait},
    tasks::{ScannEvent, background_task, ble_scanner_task},
};
use btleplug::api::CentralEvent;
use slint::Weak;
use std::env;
use tokio::sync::{
    OnceCell,
    mpsc::{self, Sender},
};

slint::include_modules!();

pub(crate) mod ble;
pub(crate) mod helpers;
pub(crate) mod models;
pub(crate) mod store_handler;
pub(crate) mod stores;
pub(crate) mod tasks;

pub enum ExtendedCentralEvent {
    Base(CentralEvent),
    Exit,
}
static CENTRAL_TX: OnceCell<Sender<ExtendedCentralEvent>> = OnceCell::const_new();
static SCAN_TX: OnceCell<Sender<ScannEvent>> = OnceCell::const_new();

pub static MAINWINDOW_WEAK: OnceCell<Weak<MainWindow>> = OnceCell::const_new();

pub trait InitTrait {
    fn init(&mut self) -> impl Future<Output = ()>;
}

pub async fn main() {
    unsafe {
        env::set_var("SLINT_SCALE_FACTOR", "1");
    }

    let (central_tx, central_rx) = mpsc::channel::<ExtendedCentralEvent>(10);

    CENTRAL_TX.set(central_tx).expect("fail to init tx");

    let main_window = MainWindow::new().expect("fail to create MainWindow");

    MAINWINDOW_WEAK
        .set(main_window.as_weak().clone())
        .map_err(|_| "fail to set main_window_weak")
        .unwrap();

    let ble_scanner_task_handle = tokio::spawn(async move { ble_scanner_task().await });
    let background_task_handle = tokio::spawn(async move { background_task(central_rx).await });

    let sidenav_store = main_window.global::<SideNavigationStore>();

    sidenav_store.on_onRescan(|| {
        if let Some(scan_tx) = SCAN_TX.get() {
            scan_tx
                .try_send(ScannEvent::Restart)
                .expect("fail to send rescan event");
        }
    });

    sidenav_store.on_onPeripheralConnect(|peripheral_id| {
        tokio::spawn(async move {
            let mut store = SIDE_NAV_STORE.lock().await;
            let _ = store.handle_on_connect(peripheral_id.into()).await;
        });
    });

    main_window.run().expect("fail to run window");

    if let Some(scan_tx) = SCAN_TX.get() {
        scan_tx
            .send(ScannEvent::Exit)
            .await
            .expect("fail to send break event");
    }

    if let Some(scan_tx) = CENTRAL_TX.get() {
        scan_tx
            .send(ExtendedCentralEvent::Exit)
            .await
            .expect("fail to send exit event");
    }

    ble_scanner_task_handle
        .await
        .expect("fail to wait ble_scanner_task");
    println!("ble_scanner_task stopped!");

    background_task_handle
        .await
        .expect("fail to wait background_task");
    println!("background_task stopped!");

    println!("success to exit");

    unsafe {
        env::remove_var("SLINT_SCALE_FACTOR");
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(_android_app: slint::android::AndroidApp) {
    todo!()
    // slint::android::init(android_app).unwrap();
    // let ui = ui();
    // MaterialWindowAdapter::get(&ui).set_disable_hover(true);
    // ui.run().unwrap();
}
