use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{Error, error, HttpRequest, HttpResponse, Responder};
use actix_web::http::StatusCode;
use crate::services::json_response::JsonResponse;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


pub async fn favicon() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./static/favicon.ico")?)
}

pub async fn page_not_found() -> impl Responder {
    JsonResponse::new().status(StatusCode::NOT_FOUND).error("Page not found")
}

pub async fn healthcheck() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, Postgres, and SQLX";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

pub async fn spa_index(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    // Check if the file exists before attempting to open it
    if let Err(err) = std::fs::metadata(&path) {
        // Return a response with the actual error message
        return Err(error::ErrorInternalServerError(format!("Error checking file: {}", err)));
    }

    Ok(NamedFile::open(path)?)
}