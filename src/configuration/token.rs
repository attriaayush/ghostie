use std::{env, fs};

use super::config::Config;

pub struct Token;

impl Token {
    pub fn is_set() -> bool {
        let token_file = &Config::read().token_file;
        if token_file.exists() {
            return !fs::read_to_string(token_file).unwrap_or_default().trim().is_empty();
        }

        env::var("GITHUB_TOKEN").map(|v| !v.is_empty()).unwrap_or(false)
    }

    pub fn set(token: String) {
        let token_file = &Config::read().token_file;
        fs::write(token_file, format!("GITHUB_TOKEN={}", &token))
            .unwrap_or_else(|err| panic!("Error writing token: {}", err));
    }

    pub fn get() -> String {
        let token_file = &Config::read().token_file;
        if token_file.exists() {
            let content =
                fs::read_to_string(token_file).unwrap_or_else(|err| panic!("Error reading token file: {}", err));
            let content = content.trim();

            // Handle both "GITHUB_TOKEN=<value>" and raw token formats
            return if let Some((_key, value)) = content.split_once('=') {
                value.to_owned()
            } else {
                content.to_owned()
            };
        }

        env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable not set")
    }
}
