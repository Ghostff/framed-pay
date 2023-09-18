use actix_web::web;
use actix_web::web::{get, post};
use crate::{controllers};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/home", get().to(controllers::home_controller::index))
        .route("/favicon.ico", get().to(controllers::home_controller::favicon))
        .route("/load", get().to(controllers::iframe_controller::load))
        .route("/load", post().to(controllers::iframe_controller::submit))
        .service(actix_files::Files::new("/assets", "./static/assets").show_files_listing().use_last_modified(true));
}

