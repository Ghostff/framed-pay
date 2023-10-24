use actix_web::web;
use actix_web_validator::JsonConfig;
use crate::{controllers};
use crate::middlewares::error_middleware::json_validation;

pub fn init(cfg: &mut web::ServiceConfig) {

    cfg.app_data(JsonConfig::default().error_handler(json_validation))
        .route("healthcheck", web::get().to(controllers::home_controller::healthcheck))
        .route("register", web::post().to(controllers::auth_controller::register))
        .route("login", web::post().to(controllers::auth_controller::login))
        .route("change-password", web::put().to(controllers::auth_controller::change_password))
        .route("reset-password", web::post().to(controllers::auth_controller::password_reset))
        .route("verify-password-reset-token", web::post().to(controllers::auth_controller::verify_password_reset_token))
        .route("logout", web::get().to(controllers::auth_controller::logout))
        .service(
            web::scope("")
                .route("user", web::get().to(controllers::user_controller::me))
                .route("dev-tools", web::get().to(controllers::user_controller::get_dev_tools))
                .route("user/generate-api-key", web::put().to(controllers::user_controller::generate_new_api_key))
                .default_service(web::to(controllers::home_controller::page_not_found))
        )
        .default_service(web::to(controllers::home_controller::page_not_found));
}