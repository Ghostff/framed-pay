use actix_web::web;
use actix_web::web::get;
use crate::{controllers};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/home", get().to(controllers::home::index))
        .route("/favicon.ico", get().to(controllers::home::favicon))
        .service(actix_files::Files::new("/assets", "./static/assets").show_files_listing().use_last_modified(true));
}

