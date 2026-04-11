use daemonize::Daemonize;
use tracing::{error, info};

use crate::daemon::proc::ProcManager;
use crate::log::Logger;

pub struct Daemon;

impl Daemon {
    pub fn daemonize<Fut>(async_process: impl Fn() -> Fut)
    where
        Fut: std::future::Future<Output = ()>,
    {
        let logger = Logger::init("ghostie_daemon");

        match ProcManager::init("ghostie").register() {
            Ok(proc) => {
                Self::run_as_daemon(proc, logger);
                tokio::runtime::Runtime::new().unwrap().block_on(async_process());
            }

            Err(err) => eprintln!("{}", err),
        }
    }

    pub fn stop_daemon() {
        ProcManager::init("ghostie").kill_process().unwrap();
        info!("ghostie background stopped by user")
    }

    pub fn show_logs() {
        Logger::init("ghostie_daemon").display_stdout();
    }

    pub fn clear_logs() {
        Logger::init("ghostie_daemon").clear_stdout().unwrap();
    }

    fn run_as_daemon(proc: ProcManager, logger: Logger) {
        let [stdout, stderr] = logger.register().unwrap();
        let daemon = Daemonize::new()
            .pid_file(proc.pid_file)
            .chown_pid_file(false)
            .stdout(stdout)
            .stderr(stderr);

        match daemon.start() {
            Ok(_) => info!("ghostie successfully started as a background process"),
            Err(e) => error!("Error: {}", e),
        }
    }
}
