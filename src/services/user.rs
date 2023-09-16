use actix_web::{web, web::Data};

use chrono::{DateTime, Utc, Duration, prelude::*};
use uuid::Uuid;

use crate::{DBPool, models::users::{User}, repositories as repo };
use crate::errors::{AuthenticationError, DatabaseError};
use crate::models::users::{VerifyPasswordResetTokenSchema};

pub async fn get_user_by_token(conn: &Data<DBPool>, payload: &VerifyPasswordResetTokenSchema) -> Result<User, Box<dyn std::error::Error>> {
    let uuid = Uuid::parse_str(payload.uid.as_str()).map_err(|_| DatabaseError::InvalidUuid)?;

    let db = conn.clone();
    let token = payload.token.clone();
    let user = web::block(move || repo::user::find_by_password_reset_token(&db, token, uuid))
        .await
        .map_err(|e| DatabaseError::DatabaseConnectionFailed(format!("{:?}", e)))??;

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
