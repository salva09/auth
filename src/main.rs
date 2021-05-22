#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use color_eyre::Result;
use tracing::{info, instrument};

use crate::config::Config;
use crate::handlers::app_config;

mod config;
mod crypto;
mod database;
mod handlers;

#[actix_rt::main]
#[instrument]
async fn main() -> Result<()> {
    Config::load_logger();

    let host = Config::get("HOST".parse()?);
    let port = Config::get("PORT".parse()?);

    info!("Starting server at http://{}:{}/", host, port);

    HttpServer::new(|| App::new().wrap(Logger::default()).configure(app_config))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await?;

    Ok(())
}
