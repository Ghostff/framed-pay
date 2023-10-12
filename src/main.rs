use std::thread;
use std::time::Duration;
use actix_cors::Cors;
use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_extensible_rate_limit::backend::SimpleInputFunctionBuilder;
use actix_extensible_rate_limit::RateLimiter;
use actix_web::{App, HttpServer, middleware, web};
use actix_web::http::{header};
use actix_web::middleware::{Logger, Compress, NormalizePath, ErrorHandlers};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use crate::config::{ENV};
use crate::services::job::Job;

mod controllers;
mod models;
mod middlewares;
mod config;
mod routes;
mod repositories;
mod services;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    dotenv().ok();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&ENV.database_url)
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let (sender, receiver) = mpsc::channel::<u8>(ENV.jobs_channel_buffer_size);
    let job = Job::new(pool.clone(), sender);

    start_job_server(job.clone(), receiver);
    start_web_server(pool, job).await
}

fn start_job_server(job: Job, mut receiver: Receiver<u8>) {
    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            while let Some(channel) = receiver.recv().await {
                job.dispatch(channel).await;
            }
        });
    });
}

async fn start_web_server(pool: Pool<Postgres>, job: Job) -> std::io::Result<()> {
    // A backend is responsible for storing rate limit data and choosing whether to allow/deny requests
    let backend = InMemoryBackend::builder().build();

    HttpServer::new(move || {
        // Assign a limit of 5 requests per minute per client ip address
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 200)
            .real_ip_key()
            .build();

        let throttle = RateLimiter::builder(backend.clone(), input)
            .add_headers()
            .build();

        let cors = Cors::permissive();

        App::new()
            .wrap(throttle)
            .wrap(
                middleware::DefaultHeaders::new()
                    .add((header::CONTENT_SECURITY_POLICY, "default-src 'self';"))
                    .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
                    .add((header::X_XSS_PROTECTION, "1; mode=block"))
                    .add((header::REFERRER_POLICY, "strict-origin-when-cross-origin"))
                    .add((header::X_FRAME_OPTIONS, "deny"))
                    .add((header::STRICT_TRANSPORT_SECURITY, "max-age=63072000; includeSubDomains; preload"))
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(job.clone()))
            .service(web::scope("/api")
                .wrap(cors)
                .configure(routes::api::init)
            )
            .configure(routes::web::init)
            .default_service(web::to(controllers::home_controller::spa_index))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            .wrap(ErrorHandlers::new().default_handler(middlewares::error_middleware::web))
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
        .bind(("0.0.0.0", 80))?
        .run()
        .await
}
// cargo watch -i "web/*" -x run
// cargo modules generate tree
// https://codevoweb.com/rust-jwt-authentication-with-actix-web/
// validation https://github.com/Keats/validator
// migration:
//  create: sqlx migrate add -r <name>
//  run: diesel migration run
//  run: diesel migration redo
//  ps aux | grep framed_pay