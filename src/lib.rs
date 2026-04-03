use std::{env, time::Duration};

use slint::Weak;
use tokio::sync::{
    OnceCell,
    mpsc::{self, Receiver, Sender},
};

use crate::{
    helpers::send_msg_type_and_val::{send_msg_and_val, try_send_msg_and_val},
    stores::{DEVICES_STORE, devices_store::DevicesStoreTrait},
};

slint::include_modules!();

pub mod ble;
pub mod helpers;
pub mod models;
pub mod stores;

pub static TX: OnceCell<Sender<MsgTypeAndVal>> = OnceCell::const_new();

pub trait InitTrait {
    fn init(&mut self) -> impl Future<Output = ()>;
}

pub enum MsgTypeAndVal {
    SetAdapters(Vec<String>),
    GetAdapters,
    ShutdownTx,
}

async fn background_task(main_window: Weak<MainWindow>, mut rx: Receiver<MsgTypeAndVal>) {
    DEVICES_STORE.lock().await.init().await;

    while let Some(msg) = rx.recv().await {
        let cloned_main_window = main_window.clone();
        match msg {
            MsgTypeAndVal::SetAdapters(adapters) => {
                println!("set adapter received");

                slint::invoke_from_event_loop(move || {
                    if let Some(main_window) = cloned_main_window.clone().upgrade() {
                        let logic = main_window.global::<Logic>();

                        let model = slint::ModelRc::new(slint::VecModel::from(
                            adapters
                                .into_iter()
                                .map(Into::into)
                                .collect::<Vec<slint::SharedString>>(),
                        ));

                        logic.set_adapter(model);
                    }
                })
                .unwrap();
            }
            MsgTypeAndVal::GetAdapters => {
                DEVICES_STORE.lock().await.get_adapter_infos().await;
            }
            MsgTypeAndVal::ShutdownTx => break,
        }
    }
}

pub fn main() {
    unsafe {
        env::set_var("SLINT_SCALE_FACTOR", "1");
    }
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let (tx, rx) = mpsc::channel::<MsgTypeAndVal>(10);

    let main_window = MainWindow::new().expect("fail to create MainWindow");

    main_window.global::<Logic>().on_get_adapter_info(move || {
        try_send_msg_and_val(MsgTypeAndVal::GetAdapters).expect("fail to get adapters");
    });

    let window_weak = main_window.as_weak();
    TX.set(tx).expect("fail to first set TX");

    let bg_handle = runtime.spawn(async move { background_task(window_weak, rx).await });

    main_window.run().expect("fail to run window");

    println!("sending ShutdownTx");
    runtime.block_on(async {
        let _ = send_msg_and_val(MsgTypeAndVal::ShutdownTx).await;
        let _ = bg_handle.await;
    });

    println!("wait shutdown...");
    runtime.shutdown_timeout(Duration::from_secs(5));
    println!("success to exit");

    unsafe {
        env::remove_var("SLINT_SCALE_FACTOR");
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) {
    slint::android::init(android_app).unwrap();
    let ui = ui();
    MaterialWindowAdapter::get(&ui).set_disable_hover(true);
    ui.run().unwrap();
}
