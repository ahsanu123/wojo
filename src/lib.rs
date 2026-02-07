slint::include_modules!();

pub mod ble;

fn ui() -> MainWindow {
    MainWindow::new().unwrap()
}

pub fn main() {
    let ui = ui();
    ui.run().unwrap();
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) {
    slint::android::init(android_app).unwrap();
    let ui = ui();
    MaterialWindowAdapter::get(&ui).set_disable_hover(true);
    ui.run().unwrap();
}
