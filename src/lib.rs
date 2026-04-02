use slint::Weak;
use tokio::sync::{
    OnceCell,
    mpsc::{self, Receiver, Sender},
};

use crate::stores::{DEVICES_STORE, devices_store::DevicesStoreTrait};

slint::include_modules!();

pub mod ble;
pub mod models;
pub mod stores;

pub static TX: OnceCell<Sender<MsgTypeAndVal>> = OnceCell::const_new();

pub trait InitTrait {
    fn init(&mut self) -> impl Future<Output = ()>;
}

pub enum MsgTypeAndVal {
    SetAdapters(Vec<String>),
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
        }
    }
}

pub fn main() {
    let (tx, rx) = mpsc::channel::<MsgTypeAndVal>(10);

    let main_window = MainWindow::new().expect("fail to create MainWindow");

    main_window.global::<Logic>().on_get_adapter_info(move || {
        tokio::spawn(async {
            println!("on_get_adapter_info");
            DEVICES_STORE.lock().await.get_adapter_infos().await;
        });
    });

    let window_weak = main_window.as_weak();

    tokio::spawn(async move {
        TX.get_or_init(async || tx).await;
        background_task(window_weak, rx).await;
    });

    main_window.run().unwrap();
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) {
    slint::android::init(android_app).unwrap();
    let ui = ui();
    MaterialWindowAdapter::get(&ui).set_disable_hover(true);
    ui.run().unwrap();
}
