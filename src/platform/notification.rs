use std::time::Duration;

#[cfg(target_os = "linux")]
use std::path::PathBuf;
#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "macos")]
use notify_rust::{Notification, Timeout};

pub struct NotificationManager {
    #[cfg(target_os = "linux")]
    send: Option<PathBuf>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "linux")]
            send: match which_crate::which("notify-send") {
                Ok(path) => Some(path),
                Err(_) => None,
            },
        }
    }

    #[allow(unused_variables)]
    pub fn send<P: AsRef<str>>(&self, message: P, timeout: Duration) {
        cfg_if::cfg_if! {
          if #[cfg(target_os = "macos")] {
            let mut notification = Notification::new();
            notification.summary("Ghostie")
              .body(message.as_ref())
              .appname("ghostie");

            notification.timeout(Timeout::Milliseconds(timeout.as_millis() as u32));
            notification.show().ok();
          } else if #[cfg(target_os = "linux")] {
            if let Some(ns) = self.send.as_ref() {
              let mut command = Command::new(ns);

              command.arg("-t");
              command.arg(format!("{}", timeout.as_millis() as u32));
              command.args(["-a", "Ghostie", "Ghostie"]);
              command.arg(message.as_ref());
              command.output().ok();
            }
          }
        }
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}
