use std::time::Duration;

#[cfg(target_os = "macos")]
use notify_rust::{Notification, Timeout};

#[allow(unused_variables)]
pub fn send<P: AsRef<str>>(message: P, timeout: Duration) {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            let mut notification = Notification::new();
            notification.summary("Ghostie")
                .body(message.as_ref())
                .appname("ghostie");

            notification.timeout(Timeout::Milliseconds(timeout.as_millis() as u32));
            notification.show().ok();
        }
    }
}
