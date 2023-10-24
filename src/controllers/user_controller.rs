use actix_web::{ HttpRequest, Responder };
use actix_web::web::Data;
use serde_json::json;
use sqlx::PgPool;

use crate::models::user_model::User;
use crate::{
    repositories,
    services,
    services::json_response::JsonResponse
};

pub async fn me(_: HttpRequest, user: User) -> impl Responder {
    JsonResponse::new().ok(user.get_filtered())
}

pub async fn generate_new_api_key(_: HttpRequest, mut user: User, conn: Data<PgPool>) -> impl Responder {
    user.api_key = match services::user::generate_api_key(&conn).await {
        Err(e) => return JsonResponse::fetal(e),
        Ok(r) => r,
    };

    match repositories::user_repository::update(&conn, &user).await {
        Err(e) => JsonResponse::fetal(e),
        Ok(_) => JsonResponse::new().ok(json!({ "api_key": user.api_key}))
    }
}

pub async fn get_dev_tools(_: HttpRequest, user: User) -> impl Responder {
    JsonResponse::new().ok(json!({
        "api_key": user.api_key
    }))
}

// make auth guard
// fix password reset