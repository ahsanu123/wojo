use crate::{CENTRAL_TX, ExtendedCentralEvent, SCAN_TX, ble::ble_manager, stores::SIDE_NAV_STORE};
use btleplug::api::{Central, Manager, ScanFilter};
use futures::StreamExt;
use tokio::sync::mpsc::{self};

pub enum ScannEvent {
    Start,
    Stop,
    Restart,
    Exit,
}

pub async fn ble_scanner_task() {
    let (scann_cmd_tx, mut scan_cmd_rx) = mpsc::channel::<ScannEvent>(10);

    SCAN_TX.set(scann_cmd_tx).expect("fail to init scann tx");

    let adapters = ble_manager().await.adapters().await.unwrap();
    let mut side_nav_store = SIDE_NAV_STORE.lock().await;
    side_nav_store.set_adapter_infos(&adapters).await;

    let central = adapters.first().unwrap();

    let mut events = central.events().await.expect("fail to get central events");
    central.start_scan(ScanFilter::default()).await.unwrap();

    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            if let Some(central_tx) = CENTRAL_TX.get() {
                central_tx
                    .send(ExtendedCentralEvent::Base(event))
                    .await
                    .expect("fail to send event");
            }
        }
    });

    while let Some(cmd) = scan_cmd_rx.recv().await {
        println!("some scan event");
        match cmd {
            ScannEvent::Exit => {
                break;
            }
            ScannEvent::Start => {
                let _ = central.start_scan(ScanFilter::default()).await;
            }
            ScannEvent::Stop => {
                let _ = central.stop_scan().await;
            }
            ScannEvent::Restart => {
                let _ = central.stop_scan().await;
                let _ = central.start_scan(ScanFilter::default()).await;
            }
        };
    }
}
