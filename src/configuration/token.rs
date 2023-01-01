use std::{env, fs};

use super::config::Config;

pub struct Token;

impl Token {
    pub fn is_set() -> bool {
        let token_file = &Config::read().token_file;
        if token_file.exists() {
            return !fs::read_to_string(token_file)
                .unwrap_or_else(|_| "".to_owned())
                .is_empty();
        }

        !env::var("GITHUB_TOKEN").unwrap_or_else(|_| "".to_owned()).is_empty()
    }

    pub fn set(token: String) {
        let token_file = &Config::read().token_file;
        fs::write(token_file, format!("GITHUB_TOKEN={}", &token)).unwrap_or_else(|err| panic!("Error: {}", err));
    }

    pub fn get() -> String {
        let token_file = &Config::read().token_file;
        if token_file.exists() {
            return fs::read_to_string(token_file)
                .unwrap_or_else(|err| panic!("Error: {}", err))
                .split('=')
                .collect::<Vec<&str>>()[1]
                .to_owned();
        }

        env::var("GITHUB_TOKEN").unwrap()
    }
}
