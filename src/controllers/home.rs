use actix_files::NamedFile;
use actix_web::{Error, HttpResponse, Responder};
use actix_web::http::StatusCode;
use crate::models::response::Response;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


pub async fn favicon() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}

pub async fn page_not_found() -> impl Responder {
    Response::new().status(StatusCode::NOT_FOUND).error("Page not found")
}

pub async fn healthcheck() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, Postgres, and SQLX";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}