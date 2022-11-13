use std::fs::{self, File};

use anyhow::Result;

pub struct Logger {
    pub stdout_file: String,
    pub stderr_file: String,
}

impl Logger {
    pub fn init(file_name: &str) -> Self {
        Self {
            stdout_file: format!("/tmp/{}.out", file_name),
            stderr_file: format!("/tmp/{}.err", file_name),
        }
    }

    pub fn register(&self) -> Result<[File; 2]> {
        let stdout = File::create(&self.stdout_file)?;
        let stderr = File::create(&self.stderr_file)?;

        Ok([stdout, stderr])
    }

    fn read_file(file: &str) -> String {
        fs::read_to_string(file).unwrap_or_default()
    }

    pub fn clear_stdout(&self) -> Result<()> {
        self.register()?;
        Ok(())
    }

    pub fn display_stdout(&self) {
        let stderr_results = Self::read_file(&self.stderr_file);
        let stdout_results = Self::read_file(&self.stdout_file);

        if stderr_results.is_empty() {
            println!("{}", stdout_results);
        } else {
            println!("{}", stderr_results);
        }
    }
}

#[macro_export]
macro_rules! info {
  ($($lvl:tt)*) => {
    println!("[{}][INFO] {}", chrono::offset::Local::now().to_rfc2822(), $($lvl)*)
  };
}

#[macro_export]
macro_rules! error {
  ($($lvl:tt)*) => {
    eprintln!("[{}][ERROR] {}", chrono::offset::Local::now().to_rfc2822(), $($lvl)*)
  };
}
