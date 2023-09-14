use std::env;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_domain: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i32,
    pub sendgrid_api_key: String,
    pub redis_url: String,
}

lazy_static! {
    pub static ref ENV: Config = {
        let jwt_max_age = env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE must be set");

        Config {
            app_domain: env::var("APP_DOMAIN").expect("APP_DOMAIN must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expires_in: env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set"),
            jwt_max_age: jwt_max_age.parse::<i32>().unwrap(),
            sendgrid_api_key: env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
        }
    };
}