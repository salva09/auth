use dotenv::dotenv;

use serde::Deserialize;

use std::env;
use tracing_subscriber::EnvFilter;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn get(key: String) -> String {
        dotenv().ok();

        env::var(&key).expect(&*format!("Key {} not set", &key))
    }

    #[instrument]
    pub fn load_logger() {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }
}
