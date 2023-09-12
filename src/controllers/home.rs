use actix_files::NamedFile;
use actix_web::{Error, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


pub async fn favicon() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}


pub async fn healthcheck() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, Postgres, and SQLX";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}