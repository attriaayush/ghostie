use std::process::Command;

pub struct Token;

impl Token {
    pub fn get() -> String {
        let output = Command::new("gh")
            .args(["auth", "token"])
            .output()
            .expect("Failed to run `gh`. Is the GitHub CLI installed? https://cli.github.com");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("gh auth failed: {}", stderr);
            eprintln!("Run `gh auth login` to authenticate.");
            std::process::exit(1);
        }

        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }
}
