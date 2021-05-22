mod models;
mod schema;

use color_eyre::Result;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::config::Config;
use crate::database::models::{NewUser, User};

fn establish_connection() -> SqliteConnection {
    let database_url = Config::get("DATABASE_URL".parse().unwrap());

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

pub fn find_user(username: String, passwd: Option<String>) -> Result<User> {
    use schema::users::dsl::*;

    let connection = establish_connection();

    let user: User = match passwd {
        Some(psw) => {
            users
                .filter(name.eq(username))
                .filter(password.eq(psw))
                .first(&connection)?
        },
        None => {
            users
                .filter(name.eq(username))
                .first(&connection)?
        },
    };

    Ok(user.clone())
}
