use actix_web::{web::Data};
use chrono::{DateTime, Utc, Duration};
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

use crate::{models::user_model::{User}, repositories as repo, services};
use crate::errors::{AuthenticationError, DatabaseError};
use crate::models::user_model::{VerifyPasswordResetTokenSchema};

pub async fn get_user_by_token(conn: &Data<PgPool>, payload: &VerifyPasswordResetTokenSchema) -> Result<User, Box<dyn std::error::Error>> {
    // extract id from uri payload/
    let uuid = Uuid::parse_str(payload.uid.as_str()).map_err(|_| DatabaseError::InvalidUuid)?;
    // find user with token and id
    let user = match repo::user_repository::find_by_password_reset_token(&conn, &uuid, &payload.token).await {
        Ok(u) => u,
        Err(DatabaseError::RecordNotFound) => return Err(Box::new(AuthenticationError::TokenNotFound)),
        Err(e) => {
            log::error!("{:?}", e);
            return Err(Box::new(AuthenticationError::Error));
        },
    };

    // match url token with db token
    let token = user.password_reset_token.as_ref().ok_or(AuthenticationError::TokenNotFound)?;
    let parts: Vec<&str> = token.split('|').collect();
    if parts.len() < 2 {
        return Err(Box::new(AuthenticationError::InvalidTokenFormat));
    }

    // check if password has expired
    let request_datetime = DateTime::parse_from_rfc3339(parts[1])
        .map_err(|_| AuthenticationError::InvalidDateTimeFormat)?
        .with_timezone(&Utc);

    let duration = Utc::now() - request_datetime;
    let twenty_four_hours = Duration::hours(24);
    if duration > twenty_four_hours {
        return Err(Box::new(AuthenticationError::TokenExpired));
    }

    Ok(user)
}

pub async fn generate_api_key(conn: &PgPool) -> Result<String, SqlxError> {
    let mut api_key;
    loop {
        api_key = services::str::get_random(100);
        match repo::user_repository::api_key_exist(&conn, &api_key).await {
            Err(e) => return Err(e),
            Ok(r) => if r == false {
                return Ok(api_key);
            },
        };
    }
}
