#[macro_use]
extern crate diesel;
extern crate dotenv;

mod config;
mod database;
mod hasher;

use crate::config::Config;
use hasher::hash;

use color_eyre::Result;
use actix_web::middleware::Logger;
use actix_web::{App, web, HttpServer};
use tracing::{info, instrument};

#[actix_rt::main]
#[instrument]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server configuration");

    info!("Starting server at http://{}:{}/", config.host, config.port);

    HttpServer::new(|| App::new().wrap(Logger::default()))
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;

    Ok(())
}
