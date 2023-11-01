use std::fmt;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, web};
use actix_web::error::{ErrorUnauthorized};
use actix_web::http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use crate::{repositories};
use crate::models::jwt::TokenClaims;
use crate::models::user::User;
use futures_util::future::LocalBoxFuture;
use sqlx::PgPool;
use crate::config::ENV;
use crate::services::cookie::{AccessTokenCookie, Cookie};

#[derive(Debug, Serialize)]
struct UnAuthorizedResponse<'a> {
    error: &'a str,
    status_code: u16,
}

impl fmt::Display for UnAuthorizedResponse<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

fn fail() -> Result<User, Error> {
    Err(ErrorUnauthorized(UnAuthorizedResponse {
        error: "Unauthorized",
        status_code: StatusCode::UNAUTHORIZED.as_u16(),
    }))
}

impl FromRequest for User {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let conn = match req.app_data::<web::Data<PgPool>>() {
            Some(data) => data.get_ref().clone(),
            None => return Box::pin(async { fail() })
        };

        let token = req
            .cookie(AccessTokenCookie::NAME)
            .map(|c| c.value().to_string());

        let token = match token {
            Some(token) => token,
            None => return Box::pin(async { fail() })
        };

        let jwt_secret = ENV.jwt_secret.as_str();

        Box::pin(async move {
            let claims = match decode::<TokenClaims>(
                &token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::default(),
            ) {
                Ok(c) => c.claims,
                Err(_) => return fail()
            };

            let user_id: uuid::Uuid = match uuid::Uuid::parse_str(claims.sub.as_str()) {
                Ok(user_id) => user_id,
                Err(_) => return fail()
            };

            match repositories::user_repository::find(&conn, &user_id).await {
               Ok(user) => Ok(user),
                _ => fail()
            }
        })
    }
}