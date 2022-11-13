use std::fs;

use anyhow::Result;
use sysinfo::{Pid, ProcessExt, System, SystemExt};

pub struct ProcManager {
    pub pid_file: String,
}

impl ProcManager {
    pub fn init(process_name: &str) -> Self {
        Self {
            pid_file: format!("/tmp/{}_pid_file", process_name),
        }
    }

    pub fn register(self) -> Result<Self> {
        Self::try_setup(&self)?;
        Ok(self)
    }

    pub fn kill_process(&self) -> Result<()> {
        let pid = self.read_pid_from_file();
        if let Some(process_id) = pid {
            if let Some(proc) = System::new_all().process(Pid::from(process_id)) {
                proc.kill();
                fs::remove_file(&self.pid_file)?;
            }
        }

        Ok(())
    }

    fn try_setup(&self) -> Result<()> {
        let pid = self.read_pid_from_file();
        let status = Self::check_existing_process(pid);
        if status {
            anyhow::bail!(
                "The background process is already running, process id: {}",
                pid.unwrap()
            );
        }

        if !status && self.read_pid_from_file().is_some() {
            fs::remove_file(&self.pid_file)?;
        }

        Ok(())
    }

    fn check_existing_process(pid: Option<i32>) -> bool {
        if let Some(process_id) = pid {
            if System::new_all().process(Pid::from(process_id)).is_some() {
                return true;
            }
        }

        false
    }

    fn read_pid_from_file(&self) -> Option<i32> {
        if let Ok(pid) = fs::read_to_string(&self.pid_file) {
            if !pid.is_empty() {
                return Some(pid.parse::<i32>().unwrap());
            }
        }
        None
    }
}
