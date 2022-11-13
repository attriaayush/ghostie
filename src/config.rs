use std::env;
use std::fs;
use std::path::PathBuf;

pub fn home_dir() -> PathBuf {
    dirs::home_dir()
        .ok_or_else(|| PathBuf::from("./"))
        .unwrap()
        .join(PathBuf::from(".ghostie"))
}

pub fn create_dir() -> PathBuf {
    let dir = home_dir();
    if !dir.exists() {
        std::fs::create_dir(dir.clone()).unwrap();
    }

    dir
}

pub fn is_token_set() -> bool {
    let token_file = home_dir().join(PathBuf::from("github.token"));
    if token_file.exists() {
        return !fs::read_to_string(token_file)
            .unwrap_or_else(|_| "".to_owned())
            .is_empty();
    }

    !env::var("GITHUB_TOKEN").unwrap_or_else(|_| "".to_owned()).is_empty()
}

pub fn write_token_to_file(token: String) {
    let token_file = create_dir().join(PathBuf::from("github.token"));
    fs::write(token_file, format!("GITHUB_TOKEN={}", &token)).unwrap_or_else(|err| panic!("Error: {}", err));
}

pub fn github_token() -> String {
    let token_file = home_dir().join(PathBuf::from("github.token"));
    if token_file.exists() {
        return fs::read_to_string(token_file)
            .unwrap_or_else(|err| panic!("Error: {}", err))
            .split('=')
            .collect::<Vec<&str>>()[1]
            .to_owned();
    }

    env::var("GITHUB_TOKEN").unwrap()
}
