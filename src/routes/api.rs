use actix_web::web;
use actix_web_validator::JsonConfig;
use crate::{controllers, models};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.app_data(JsonConfig::default().error_handler(models::response::Response::error_handler))
        .route("healthcheck", web::get().to(controllers::home::healthcheck))
        .route("register", web::post().to(controllers::auth::register))
        .route("login", web::post().to(controllers::auth::login))
        .route("reset-password", web::post().to(controllers::auth::reset_password))
        .route("logout", web::get().to(controllers::auth::logout))
        .service(
            web::scope("")
                .route("user", web::get().to(controllers::user::me))
                .default_service(web::to(controllers::home::page_not_found))
        )
        .default_service(web::to(controllers::home::page_not_found));
}