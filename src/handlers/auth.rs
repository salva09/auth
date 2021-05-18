use actix_web::{HttpResponse, Responder};

pub async fn create_user() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn user_login() -> impl Responder {
    HttpResponse::Ok()
}
