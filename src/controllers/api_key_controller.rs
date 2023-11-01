use actix_web::{HttpRequest, Responder, web};
use actix_web::web::Data;
use actix_web_validator::Json;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use validator::HasLen;

use crate::models::user::User;
use crate::{repositories, services, services::json_response::JsonResponse};
use crate::models::api_key::{CreateApiKeySchema};

pub async fn create(_: HttpRequest, body: Json<CreateApiKeySchema>, user: User, conn: Data<PgPool>) -> impl Responder {
    match services::user_service::attach_api_key(&conn, &user, &body.name).await {
        Err(e) => JsonResponse::fetal(e),
        Ok(api_key) => JsonResponse::new().ok(json!(api_key.get_filtered()))
    }
}

pub async fn delete(_: HttpRequest, info: web::Path<(Uuid,)>, user: User, conn: Data<PgPool>) -> impl Responder {
    match repositories::api_key_repository::find_by_user_id(&conn, &user.id).await {
        Ok(keys) => {
            if keys.length() == 0 {
                return JsonResponse::new().error("you must have at least one api key.")
            }
        },
        Err(e) => return JsonResponse::fetal(e)
    };

    let url_params = info.into_inner();
    match repositories::api_key_repository::delete(&conn, &url_params.0, &user.id).await {
        Ok(_) => JsonResponse::new().ok("success"),
        Err(e) => JsonResponse::fetal(e)
    }
}
