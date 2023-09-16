use actix_web::web;
use actix_web_validator::JsonConfig;
use crate::{controllers, services};

pub fn init(cfg: &mut web::ServiceConfig) {


    cfg.app_data(JsonConfig::default().error_handler(services::response::Response::error_handler))
        .route("healthcheck", web::get().to(controllers::home::healthcheck))
        .route("register", web::post().to(controllers::auth::register))
        .route("login", web::post().to(controllers::auth::login))
        .route("change-password", web::put().to(controllers::auth::change_password))
        .route("reset-password", web::post().to(controllers::auth::password_reset))
        .route("verify-password-reset-token", web::post().to(controllers::auth::verify_password_reset_token))
        .route("logout", web::get().to(controllers::auth::logout))
        .service(
            web::scope("")
                .route("user", web::get().to(controllers::user::me))
                .default_service(web::to(controllers::home::page_not_found))
        )
        .default_service(web::to(controllers::home::page_not_found));
}