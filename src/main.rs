use std::process::exit;

use dialoguer::{theme::ColorfulTheme, Confirm, Password};

use ghostie::{cli, configuration::token::Token};

fn prompt_token_flow() {
    let github_token = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Github token (no bearer/token prefix)")
        .allow_empty_password(false)
        .interact()
        .unwrap();

    match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like ghostie to persist the token? (required)")
        .default(true)
        .interact_opt()
        .unwrap()
    {
        Some(true) => {
            Token::set(github_token);
            println!("Successfully persisted token in ~/.ghostie/github.token")
        }
        Some(false) => {
            eprintln!("ghostie requires the token to be persisted :(");
            exit(1);
        }
        None => unreachable!(),
    }
}

fn main() {
    if !Token::is_set() {
        prompt_token_flow();
    }

    cli::init();
}
