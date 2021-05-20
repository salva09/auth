use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::crypto::hasher::hash;
use crate::database::insert_user;

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
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

pub async fn user_login() -> impl Responder {
    HttpResponse::Ok()
}
