mod models;
mod schema;

use crate::config::Config;
use crate::database::models::NewUser;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    let config = Config::from_env().expect("Server configuration");

    SqliteConnection::establish(&config.database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_user<'a>(name: &'a str, email: &'a str, password: &'a str) -> usize {
    use schema::users;

    let connection = establish_connection();
    let new_user = NewUser {
        name,
        email,
        password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&connection)
        .expect("User insertion into database")
}
