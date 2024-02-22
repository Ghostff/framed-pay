use actix_web::{Responder, web::Data};
use chrono::{DateTime, Utc, Duration};
use serde_json::json;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{models::user::{User}, repositories as repo, services};
use crate::errors::{AuthenticationError, DatabaseError};
use crate::models::api_key::ApiKey;
use crate::models::user::{VerifyPasswordResetTokenSchema};
use crate::services::json_response::JsonResponse;

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

pub async fn attach_api_key(conn: &Data<PgPool>, user: &User, name: &str) -> Result<ApiKey, DatabaseError> {
    let api_key = match services::api_key_service::generate_key(&conn).await {
        Err(e) => return Err(e),
        Ok(r) => r,
    };

    match repo::api_key_repository::create(conn, name, &api_key, &user.id).await {
        Ok(api_key) => Ok(api_key),
        Err(e) => Err(e)
    }
}

#[macro_export]
macro_rules! check_duplicate_email {
    ($conn:expr, $email:expr, $conflict_msg:expr) => {
        {
            let exist = match crate::repositories::user_repository::email_exist(&$conn, &$email).await {
                Err(e) => return crate::services::json_response::JsonResponse::fetal(e),
                Ok(r) => r,
            };

            if exist {
                return crate::services::json_response::JsonResponse::new().status(actix_web::http::StatusCode::CONFLICT).error(&$conflict_msg);
            }
        }
    };
}
