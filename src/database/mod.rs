mod models;
mod schema;

use crate::database::models::NewUser;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
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
