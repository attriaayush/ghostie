use std::{collections::HashMap, fs, path::PathBuf};

pub struct AdditionalConfig {
    polling_interval_minutes: u32,
    polling_window_days: u32,
    enable_os_notifications: bool,
}

impl Default for AdditionalConfig {
    fn default() -> Self {
        Self {
            polling_interval_minutes: 1,
            polling_window_days: 2,
            enable_os_notifications: true,
        }
    }
}

impl AdditionalConfig {
    fn load(config_file: PathBuf) -> AdditionalConfig {
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
            polling_interval_minutes: map
                .get("polling_interval_minutes")
                .unwrap_or(&"1".to_string())
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

    pub fn get_polling_interval_minutes(&self) -> u32 {
        self.polling_interval_minutes
    }

    pub fn get_enable_os_notifications(&self) -> bool {
        self.enable_os_notifications
    }
}

pub struct Config {
    pub cache_file: PathBuf,
    pub token_file: PathBuf,
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
          additional_config = AdditionalConfig::load(additional_config_path);
        }

        Config {
          cache_file: home_dir.join("notifications.db"),
          token_file: home_dir.join("github.token"),
          additional_config,
        }
    };
}

impl Config {
    pub fn read() -> &'static Self {
        &CONFIG
    }
}
