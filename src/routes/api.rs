use actix_web::web;
use crate::controllers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/healthcheck", web::get().to(controllers::home::healthcheck))
        .route("/register", web::post().to(controllers::auth::register))
        .route("/login", web::post().to(controllers::auth::login))
        .route("/logout", web::get().to(controllers::auth::logout))
        .route("/user", web::get().to(controllers::user::me));
}