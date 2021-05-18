mod auth;

use actix_web::web;
use crate::handlers::auth::{create_user,  user_login};

pub fn app_config(config: &mut web::ServiceConfig) {
    let signup = web::resource("/signup").route(web::post().to(create_user));

    let login = web::resource("/login").route(web::post().to(user_login));

    config.service(signup).service(login);
}
