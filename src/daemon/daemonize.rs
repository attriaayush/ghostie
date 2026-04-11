use std::fs::{self, File};

use anyhow::Result;
use daemonize::Daemonize;
use regex::Regex;
use tracing::{error, info};

use crate::daemon::proc::ProcManager;

struct Logger {
    stdout_file: String,
    stderr_file: String,
}

impl Logger {
    fn init(file_name: &str) -> Self {
        Self {
            stdout_file: format!("/tmp/{}.out", file_name),
            stderr_file: format!("/tmp/{}.err", file_name),
        }
    }

    fn register(&self) -> Result<[File; 2]> {
        let stdout = File::create(&self.stdout_file)?;
        let stderr = File::create(&self.stderr_file)?;
        Ok([stdout, stderr])
    }

    fn read_file(file: &str) -> String {
        fs::read_to_string(file).unwrap_or_default()
    }

    fn clear_stdout(&self) -> Result<()> {
        self.register()?;
        Ok(())
    }

    fn display_stdout(&self) {
        let re = Regex::new(r"\[(.*?)\]").unwrap();
        let capture_brackets = Regex::new("\\[|\\]").unwrap();

        let all_logs = format!(
            "{}\n{}",
            Self::read_file(&self.stderr_file).trim(),
            Self::read_file(&self.stdout_file).trim()
        );

        let mut logs: Vec<_> = all_logs
            .split('\n')
            .filter_map(|s| {
                if re.is_match(s) {
                    let timestamp = re.captures(s)?.get(0).map_or("", |m| m.as_str());
                    let parsed =
                        chrono::DateTime::parse_from_rfc2822(&capture_brackets.replace_all(timestamp, "")).ok()?;
                    Some((parsed.with_timezone(&chrono::Utc), s))
                } else if !s.is_empty() {
                    Some((chrono::offset::Utc::now(), s))
                } else {
                    None
                }
            })
            .collect();
        logs.sort_by(|a, b| a.0.cmp(&b.0));
        for log in logs.iter() {
            println!("{}", log.1)
        }
    }
}

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
