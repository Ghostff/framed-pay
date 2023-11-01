use actix_web::{ HttpRequest, Responder };
use actix_web::web::Data;
use serde_json::json;
use sqlx::{PgPool};

use crate::models::user::User;
use crate::{repositories, services::json_response::JsonResponse};
use crate::models::api_key::{ApiKey, FilteredApiKey};

pub async fn me(_: HttpRequest, user: User) -> impl Responder {
    JsonResponse::new().ok(user.get_filtered())
}

pub async fn get_dev_tools(_: HttpRequest, conn: Data<PgPool>, user: User) -> impl Responder {

    let api_keys = match repositories::api_key_repository::find_by_user_id(&conn, &user.id).await {
        Ok(keys) => keys.iter().map(|api_key: &ApiKey| api_key.get_filtered()).collect::<Vec<FilteredApiKey>>(),
        Err(e) => return JsonResponse::fetal(e),
    };

    JsonResponse::new().ok(json!({
        "api_keys": api_keys
    }))
}