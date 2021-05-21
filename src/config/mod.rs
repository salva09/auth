use dotenv::dotenv;

use serde::Deserialize;

use std::env;

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
}
