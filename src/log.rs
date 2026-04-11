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
