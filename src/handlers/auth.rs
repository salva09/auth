use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::crypto::{hasher::hash, token::generate_token};
use crate::database::{find_user, insert_user};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    name: String,
    password: String,
}

#[derive(Serialize)]
struct Result {
    message: String,
}

pub async fn create_user(user: web::Json<User>) -> impl Responder {
    let hashed_passwd = hash(user.password.as_str());
    insert_user(
        user.name.as_str(),
        user.email.as_str(),
        hashed_passwd.as_str(),
    );
    HttpResponse::Ok()
}

pub async fn user_login(user: web::Json<UserLogin>) -> impl Responder {
    match find_user(user.name.clone(), hash(user.password.as_str())) {
        Ok(user) => web::Json(Result {
            message: generate_token(user.id.to_string(), user.name).unwrap(),
        }),
        Err(_) => web::Json(Result {
            message: "User or password are incorrect".to_string(),
        }),
    }
}
