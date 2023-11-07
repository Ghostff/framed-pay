use std::env;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_name: String,
    pub app_url: String,

    pub app_domain: String,
    pub database_url: String,

    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i32,

    pub redis_url: String,

    pub mail_host: String,
    pub mail_port: u16,
    pub mail_username: String,
    pub mail_password: String,

    pub google_api_key: String,
    pub google_recaptcha_key: String,

    pub jobs_channel_buffer_size: usize,
    pub max_concurrent_email_job_task: usize,
}

lazy_static! {
    pub static ref ENV: Config = {
        let jwt_max_age = env::var("JWT_MAX_AGE").expect("JWT_MAX_AGE must be set");
        let mail_port = env::var("MAIL_PORT").expect("MAIL_PORT must be set");

        let jobs_channel_buffer_size = env::var("JOBS_CHANNEL_BUFFER_SIZE").expect("JOBS_CHANNEL_BUFFER_SIZE must be set");
        let max_concurrent_email_job_task = env::var("MAX_CONCURRENT_EMAIL_JOB_TASK").expect("MAX_CONCURRENT_EMAIL_JOB_TASK must be set");

        Config {
            app_name: env::var("APP_NAME").expect("APP_NAME must be set"),
            app_url: env::var("APP_URL").expect("APP_URL must be set"),
            app_domain: env::var("APP_DOMAIN").expect("APP_DOMAIN must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expires_in: env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set"),
            jwt_max_age: jwt_max_age.parse::<i32>().unwrap(),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            mail_host: env::var("MAIL_HOST").expect("MAIL_HOST must be set"),
            mail_username: env::var("MAIL_USERNAME").expect("MAIL_USERNAME must be set"),
            mail_port: mail_port.parse::<u16>().unwrap(),
            mail_password: env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD must be set"),
            google_api_key: env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set"),
            google_recaptcha_key: env::var("GOOGLE_RECAPTCHA_KEY").expect("GOOGLE_RECAPTCHA_KEY must be set"),
            jobs_channel_buffer_size: jobs_channel_buffer_size.parse::<usize>().unwrap(),
            max_concurrent_email_job_task: max_concurrent_email_job_task.parse::<usize>().unwrap(),
        }
    };
}