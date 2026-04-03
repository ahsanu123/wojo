use btleplug::api::{Central, CentralEvent, Manager, ScanFilter};
use futures::{StreamExt, stream::select};
use tokio::sync::{
    OnceCell,
    mpsc::{self, Receiver, Sender},
};

use crate::{ble::ble_manager, stores::DEVICES_STORE};
use std::env;

slint::include_modules!();

pub mod ble;
pub mod helpers;
pub mod models;
pub mod stores;

pub enum ScannEvent {
    Start,
    Stop,
    Restart,
}

static CENTRAL_TX: OnceCell<Sender<CentralEvent>> = OnceCell::const_new();
static SCANN_TX: OnceCell<Sender<ScannEvent>> = OnceCell::const_new();

pub trait InitTrait {
    fn init(&mut self) -> impl Future<Output = ()>;
}

async fn ble_scanner_task() {
    let (scann_cmd_tx, mut scann_cmd_rx) = mpsc::channel::<ScannEvent>(10);

    SCANN_TX.set(scann_cmd_tx).expect("fail to init scann tx");

    let adapters = ble_manager().await.adapters().await.unwrap();
    let central = adapters.first().unwrap();

    let mut events = central.events().await.expect("fail to get central events");
    central.start_scan(ScanFilter::default()).await.unwrap();

    loop {
        tokio::select! {
            maybe_event = events.next() => {
                if let Some(central_tx) = CENTRAL_TX.get() && let Some(event) = maybe_event{
                    central_tx.send(event).await.expect("fail to send event");
                }
            }

            scann_cmd = scann_cmd_rx.recv() => {
                if let Some(cmd) = scann_cmd {
                    match cmd {
                        ScannEvent::Start => {
                            central.start_scan(ScanFilter::default()).await.unwrap();
                        }
                        ScannEvent::Stop => {
                            central.stop_scan().await.unwrap();
                        },
                        ScannEvent::Restart => {
                            central.stop_scan().await.unwrap();
                            central.start_scan(ScanFilter::default()).await.unwrap();
                        },
                    }
                }
            }
        }
    }
}

async fn background_task(mut rx: Receiver<CentralEvent>) {
    while let Some(event) = rx.recv().await {
        match event {
            CentralEvent::DeviceDiscovered(peripheral_id) => println!("todo"),
            CentralEvent::DeviceUpdated(peripheral_id) => println!("todo"),
            CentralEvent::DeviceConnected(peripheral_id) => println!("todo"),
            CentralEvent::DeviceDisconnected(peripheral_id) => println!("todo"),
            CentralEvent::DeviceServicesModified(peripheral_id) => println!("todo"),
            CentralEvent::ServiceDataAdvertisement { id, service_data } => println!("todo"),
            CentralEvent::ServicesAdvertisement { id, services } => println!("todo"),
            CentralEvent::RssiUpdate { id, rssi } => println!("todo"),
            CentralEvent::StateUpdate(central_state) => println!("todo"),
            CentralEvent::ManufacturerDataAdvertisement {
                id,
                manufacturer_data,
            } => println!("todo"),
        }
    }
}

pub async fn main() {
    unsafe {
        env::set_var("SLINT_SCALE_FACTOR", "1");
    }

    let (central_tx, central_rx) = mpsc::channel::<CentralEvent>(10);

    CENTRAL_TX.set(central_tx).expect("fail to init tx");

    let main_window = MainWindow::new().expect("fail to create MainWindow");
    let main_window_weak = main_window.as_weak().clone();

    let mut locked = DEVICES_STORE.lock().await;
    locked.set_weak_main_window(main_window_weak);

    let _ble_scanner_task_handle = tokio::spawn(async move { ble_scanner_task().await });
    let _background_task_handle = tokio::spawn(async move { background_task(central_rx).await });

    main_window.run().expect("fail to run window");
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
