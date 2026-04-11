use std::{fs, path::PathBuf, sync::LazyLock};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct AdditionalConfig {
    pub polling_interval_seconds: u32,
    pub polling_window_days: u32,
    pub enable_os_notifications: bool,
}

impl Default for AdditionalConfig {
    fn default() -> Self {
        Self {
            polling_interval_seconds: 60,
            polling_window_days: 2,
            enable_os_notifications: true,
        }
    }
}

impl AdditionalConfig {
    fn load(config_file: &PathBuf) -> AdditionalConfig {
        let config_content = fs::read_to_string(config_file).unwrap_or_default();

        // Try TOML format first
        if let Ok(config) = toml::from_str::<AdditionalConfig>(&config_content) {
            return config;
        }

        // Fallback: legacy key=value format for backward compatibility
        Self::load_legacy(&config_content)
    }

    fn load_legacy(content: &str) -> AdditionalConfig {
        let mut config = AdditionalConfig::default();

        for line in content.lines() {
            let line = line.trim();
            if !line.contains('=') || line.starts_with("//") || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                match key.trim() {
                    "polling_interval_seconds" => {
                        if let Ok(v) = value.trim().parse() {
                            config.polling_interval_seconds = v;
                        }
                    }
                    "polling_window_days" => {
                        if let Ok(v) = value.trim().parse() {
                            config.polling_window_days = v;
                        }
                    }
                    "enable_os_notifications" => {
                        if let Ok(v) = value.trim().parse() {
                            config.enable_os_notifications = v;
                        }
                    }
                    _ => {}
                }
            }
        }

        config
    }
}

pub struct Config {
    pub cache_file: PathBuf,
    pub config_file: PathBuf,
    pub additional_config: AdditionalConfig,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("./")).join(".ghostie");

    if !home_dir.exists() {
        std::fs::create_dir(&home_dir).unwrap();
    }

    let additional_config_path = home_dir.join("ghostie.config");
    let additional_config = if additional_config_path.exists() {
        AdditionalConfig::load(&additional_config_path)
    } else {
        AdditionalConfig::default()
    };

    Config {
        cache_file: home_dir.join("notifications.db"),
        config_file: additional_config_path,
        additional_config,
    }
});

impl Config {
    pub fn read() -> &'static Self {
        &CONFIG
    }

    pub fn edit_additional_config() {
        let config_file = &CONFIG.config_file;
        if !config_file.exists() {
            let toml_content = "\
# Frequency of polling notifications from Github in seconds
polling_interval_seconds = 60

# Number of days to be used as polling window
polling_window_days = 2

# OS specific notifications/alerts
enable_os_notifications = true
";

            fs::write(config_file, toml_content).unwrap();
        }

        open::that(config_file).unwrap();
    }
}
