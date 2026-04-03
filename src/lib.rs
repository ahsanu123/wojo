use crate::stores::DEVICES_STORE;
use std::env;

slint::include_modules!();

pub mod ble;
pub mod helpers;
pub mod models;
pub mod stores;

pub trait InitTrait {
    fn init(&mut self) -> impl Future<Output = ()>;
}

pub async fn main() {
    unsafe {
        env::set_var("SLINT_SCALE_FACTOR", "1");
    }

    let main_window = MainWindow::new().expect("fail to create MainWindow");
    let main_window_weak = main_window.as_weak().clone();

    let mut locked = DEVICES_STORE.lock().await;
    locked.set_weak_main_window(main_window_weak);
    locked.init().await;

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
