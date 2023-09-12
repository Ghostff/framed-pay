use std::path::PathBuf;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{App, error, Error, HttpRequest, HttpServer, web};
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web_validator::JsonConfig;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;

mod controllers;
mod models;
mod middlewares;
mod config;
mod routes;

async fn spa_index(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    // Check if the file exists before attempting to open it
    if let Err(err) = std::fs::metadata(&path) {
        // Return a response with the actual error message
        return Err(error::ErrorInternalServerError(format!("Error checking file: {}", err)));
    }

    Ok(NamedFile::open(path)?)
}

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials()
            .max_age(3600);


        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
            }))
            .service(
                web::scope("/api")
                    .app_data(JsonConfig::default().error_handler(models::response::Response::error_handler))
                    .wrap(cors)
                    .configure(routes::api::init)
            )
            .configure(routes::web::init)
            .default_service(web::to(spa_index))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
        .bind(("127.0.0.1", 80))?
        .run()
        .await
}

// cargo watch -i "web/*" -x run
// cargo modules generate tree
// https://codevoweb.com/rust-jwt-authentication-with-actix-web/
// validation https://github.com/Keats/validator
// migration:
//  create: sqlx migrate add -r <name>
//  run: sqlx migrate revert
//  run: sqlx migrate run
