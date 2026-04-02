use crate::stores::DEVICES_STORE;

slint::include_modules!();

pub mod ble;
pub mod models;
pub mod stores;

pub trait InitTrait {
    fn init(&mut self) -> impl Future<Output = ()>;
}

async fn background_task() {
    DEVICES_STORE.lock().await.init().await;
}

pub fn main() {
    let main_window = MainWindow::new().expect("fail to create MainWindow");

    main_window.global::<Logic>();

    tokio::spawn(async {
        background_task().await;
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
