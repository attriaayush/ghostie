use std::path::PathBuf;

pub struct Config {
    pub cache_file: PathBuf,
    pub token_file: PathBuf,
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

        Config {cache_file: home_dir.join("notifications.db"), token_file: home_dir.join("github.token") }
    };
}

impl Config {
    pub fn init() -> &'static Config {
        &*CONFIG
    }
}
