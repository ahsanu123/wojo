use crate::MainWindow;
use slint::{EventLoopError, Weak};

pub fn set_ui_state<F>(weak_window: &Weak<MainWindow>, setter_fn: F) -> Result<(), EventLoopError>
where
    F: FnOnce(MainWindow) + Send + 'static,
{
    let weak = weak_window.clone();
    slint::invoke_from_event_loop(move || {
        if let Some(main_window) = weak.upgrade() {
            setter_fn(main_window)
        }
    })
}
