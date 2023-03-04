use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use tracing_subscriber::EnvFilter;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env();
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub rest_port: i32,
    pub graphql_port: i32,
    pub session_name: String,
    pub session_key: String,
    pub session_timeout: i64,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();

        match envy::from_env::<Config>() {
            Ok(config) => {
                config
            }
            Err(error) => panic!("Server configuration exception: {:#?}", error),
        }
    }
}

// Initiate the tracing subscriber for RUST_LOG
pub fn start_tracing() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}