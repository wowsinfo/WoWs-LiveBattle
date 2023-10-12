use env_logger::Env;

pub mod parser;
pub mod reader;
pub mod server;

pub fn setup_logger(debug: &str, release: &str) {
    if cfg!(debug_assertions) {
        env_logger::Builder::from_env(Env::default().default_filter_or(debug)).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or(release)).init();
    }
}
