use std::path::PathBuf;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{App, error, Error, HttpRequest, HttpServer, web};
use actix_web::http::{header};
use actix_web::middleware::{Logger, Compress, NormalizePath};
use apalis::prelude::{Monitor, WithStorage, WorkerBuilder, WorkerFactoryFn};
use apalis::redis::RedisStorage;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use futures_util::future;

use crate::config::{ENV};
use crate::services::email::Email;

mod controllers;
mod models;
mod middlewares;
mod config;
mod routes;
mod repositories;
mod services;
mod templates;
mod errors;

async fn spa_index(_req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    // Check if the file exists before attempting to open it
    if let Err(err) = std::fs::metadata(&path) {
        // Return a response with the actual error message
        return Err(error::ErrorInternalServerError(format!("Error checking file: {}", err)));
    }

    Ok(NamedFile::open(path)?)
}

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();

    let manager = ConnectionManager::<PgConnection>::new(ENV.database_url.clone());
    let pool: DBPool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");
    let queue = RedisStorage::connect(ENV.redis_url.clone()).await.expect("Could not connect to redis storage");

    println!("🚀 Server started successfully");

    let queue_data = queue.clone();
    let http = async {
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
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(queue_data.clone()))
                .service(web::scope("/api").wrap(cors).configure(routes::api::init))
                .configure(routes::web::init)
                .default_service(web::to(spa_index))
                .wrap(Logger::default())
                .wrap(Compress::default())
                .wrap(NormalizePath::trim())
                .wrap(Logger::new("%a %{User-Agent}i"))
        })
            .bind(("127.0.0.1", 80))?
            .run()
            .await?;

        Ok(())
    };

    let worker = Monitor::new()
        .register_with_count(2, move |index| {
            WorkerBuilder::new(format!("email-worker-{index}"))
                .with_storage(queue.clone())
                .build_fn(Email::run)
        })
        .run();

    future::try_join(http, worker).await?;

    Ok(())
}
// cargo watch -i "web/*" -x run
// cargo modules generate tree
// https://codevoweb.com/rust-jwt-authentication-with-actix-web/
// validation https://github.com/Keats/validator
// migration:
//  create: sqlx migrate add -r <name>
//  run: diesel migration run
//  run: diesel migration redo
