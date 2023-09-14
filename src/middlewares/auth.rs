use std::fmt;
use actix_web::{dev::Payload, Error, FromRequest, http, HttpRequest, web};
use actix_web::error::{ErrorUnauthorized};
use actix_web::http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use crate::{DBPool, repositories};
use crate::models::jwt::TokenClaims;
use crate::models::users::User;
use futures_util::future::LocalBoxFuture;
use crate::config::ENV;

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
        let conn = match req.app_data::<web::Data<DBPool>>() {
            Some(data) => data.get_ref(),
            None => return Box::pin(async { fail() })
        };

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .and_then(|h| h.to_str().ok())
                    .map(|h| h[7..].to_string())
            });

        let token = match token {
            Some(token) => token,
            None => return Box::pin(async { fail() })
        };

        let jwt_secret = ENV.jwt_secret.clone();
        let db = conn.clone();

        Box::pin(async move {
            let claims = match decode::<TokenClaims>(
                &token,
                &DecodingKey::from_secret(jwt_secret.as_ref()),
                &Validation::default(),
            ) {
                Ok(c) => c.claims,
                Err(_) => return fail()
            };

            let user_id: uuid::Uuid = match uuid::Uuid::parse_str(claims.sub.as_str()) {
                Ok(user_id) => user_id,
                Err(_) => return fail()
            };

            let user_result = web::block(move || repositories::user::get_by_id(db, user_id)).await;

            match user_result {
                Ok(Ok(user)) => Ok(user),
                _ => fail()
            }
        })
    }

}