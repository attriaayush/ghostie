use tracing_subscriber::EnvFilter;

use ghostie::cli;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    cli::init();
}
