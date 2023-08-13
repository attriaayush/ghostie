use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use opener::open;

pub struct AdditionalConfig {
    polling_interval_seconds: u32,
    polling_window_days: u32,
    enable_os_notifications: bool,
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
        let mut map = HashMap::new();
        let config_content = fs::read_to_string(config_file).unwrap();

        for line in config_content.lines() {
            if !line.contains('=') {
                continue;
            }
            let config = line.split('=').collect::<Vec<&str>>().to_owned();
            map.insert(config[0].to_string(), config[1].to_string());
        }

        AdditionalConfig {
            polling_interval_seconds: map
                .get("polling_interval_seconds")
                .unwrap_or(&"60".to_string())
                .parse::<u32>()
                .unwrap(),

            polling_window_days: map
                .get("polling_window_days")
                .unwrap_or(&"2".to_string())
                .parse::<u32>()
                .unwrap(),

            enable_os_notifications: map
                .get("enable_os_notifications")
                .unwrap_or(&"true".to_string())
                .parse::<bool>()
                .unwrap(),
        }
    }

    pub fn get_polling_window_days(&self) -> u32 {
        self.polling_window_days
    }

    pub fn get_polling_interval_seconds(&self) -> u32 {
        self.polling_interval_seconds
    }

    pub fn get_enable_os_notifications(&self) -> bool {
        self.enable_os_notifications
    }
}

pub struct Config {
    pub cache_file: PathBuf,
    pub token_file: PathBuf,
    pub config_file: PathBuf,
    pub additional_config: AdditionalConfig,
}

lazy_static::lazy_static! {
   pub static ref CONFIG: Config = {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| PathBuf::from("./"))
            .unwrap()
            .join(PathBuf::from(".ghostie"));

        if !home_dir.exists() {
            std::fs::create_dir(home_dir.clone()).unwrap();
        }

        let additional_config_path = home_dir.join("ghostie.config");
        let mut additional_config = AdditionalConfig::default();
        if additional_config_path.exists() {
          additional_config = AdditionalConfig::load(&additional_config_path);
        }

        Config {
          cache_file: home_dir.join("notifications.db"),
          token_file: home_dir.join("github.token"),
          config_file: additional_config_path,
          additional_config,
        }
    };
}

impl Config {
    pub fn read() -> &'static Self {
        &CONFIG
    }

    pub fn edit_additional_config() {
        let config_file = &CONFIG.config_file;
        if !config_file.exists() {
            Write::write_all(
                &mut fs::File::create(config_file).unwrap(),
                r#"// Frequency of polling notifications from Github in seconds
polling_interval_seconds=60

// Number of days to be used as polling window
polling_window_days=2

// OS specific notifications/alerts
enable_os_notifications=true"#
                    .to_string()
                    .as_bytes(),
            )
            .unwrap();
        }

        open(config_file).unwrap();
    }
}
