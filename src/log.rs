use std::fs::{self, File};

use anyhow::Result;
use regex::Regex;

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
        let re = Regex::new(r"\[(.*?)\]").unwrap();
        let capture_brackets = Regex::new("\\[|\\]").unwrap();

        let all_logs = format!(
            "{}\n{}",
            Self::read_file(&self.stderr_file).trim(),
            Self::read_file(&self.stdout_file).trim()
        );

        let mut logs: Vec<_> = all_logs
            .split('\n')
            .map(|s| {
                if re.is_match(s) {
                    let timestamp = re.captures(s).unwrap().get(0).map_or("", |m| m.as_str());
                    return (
                        chrono::DateTime::parse_from_rfc2822(&capture_brackets.replace_all(timestamp, ""))
                            .unwrap()
                            .with_timezone(&chrono::Utc),
                        s,
                    );
                }

                (chrono::offset::Utc::now(), s)
            })
            .collect();
        logs.sort_by(|a, b| a.0.cmp(&b.0));
        for log in logs.iter() {
            println!("{}", log.1)
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
macro_rules! warn {
  ($($lvl:tt)*) => {
    println!("[{}][WARN] {}", chrono::offset::Local::now().to_rfc2822(), $($lvl)*)
  };
}

#[macro_export]
macro_rules! error {
  ($($lvl:tt)*) => {
    eprintln!("[{}][ERROR] {}", chrono::offset::Local::now().to_rfc2822(), $($lvl)*)
  };
}
