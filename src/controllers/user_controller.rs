use actix_web::{HttpRequest, Responder };
use actix_web::web::Data;
use actix_web_validator::Json;
use log::error;
use phonenumber::country::Id;
use serde_json::json;
use sqlx::{PgPool};

use crate::models::user::User;
use crate::{check_bot, check_duplicate_email, repositories as repo, services::json_response::JsonResponse};
use crate::models::api_key::{ApiKey, FilteredApiKey};
use crate::utilities::crypto::verify_password;

pub async fn me(_: HttpRequest, user: User) -> impl Responder {
    JsonResponse::new().ok(user.get_filtered())
}

pub async fn update(body: Json<crate::models::user::UpdateUserSchema>, conn: Data<PgPool>, mut user: User) -> impl Responder {
    check_bot!(body);
    if user.email != body.email {
        check_duplicate_email!(&conn, &body.email, "Email already exist in our system.");
        user.email = body.email.clone();
    }

    if body.current_password.is_some() && body.new_password.is_some() {
        if !verify_password(&user.password, &body.current_password.clone().unwrap()) {
            return JsonResponse::bad_request("current_password", "Invalid password.")
        }

        user.password = body.new_password.clone().unwrap();
    }

    user.first_name = body.first_name.clone();
    user.last_name = body.last_name.clone();

    if body.phone.is_some() && body.phone != user.phone {
        let invalid_msg = "Invalid phone/country code.";

        if body.country_code.is_none() {
            return JsonResponse::bad_request("phone", invalid_msg);
        }

        let country_code = match body.country_code.clone().unwrap().parse::<Id>() {
            Ok(id) => id,
            Err(_) => return JsonResponse::bad_request("phone", invalid_msg),
        };

        let phone = body.phone.clone().unwrap().chars().filter(|&c| c.is_digit(10) || c == '+').collect();
        let number = match phonenumber::parse(Some(country_code), &phone) {
            Ok(phone) => phone,
            Err(e) => return JsonResponse::bad_request("phone", invalid_msg),
        };

        if !phonenumber::is_valid(&number) {
            return JsonResponse::bad_request("phone", invalid_msg);
        }

        user.phone = Some(phone);
    }

    if let Err(e) = repo::user_repository::update(&conn, &user).await {
        return JsonResponse::fetal(e)
    }

    JsonResponse::success()
}

pub async fn get_dev_tools(_: HttpRequest, conn: Data<PgPool>, user: User) -> impl Responder {

    let api_keys = match repo::api_key_repository::find_by_user_id(&conn, &user.id).await {
        Ok(keys) => keys.iter().map(|api_key: &ApiKey| api_key.get_filtered()).collect::<Vec<FilteredApiKey>>(),
        Err(e) => return JsonResponse::fetal(e),
    };

    JsonResponse::new().ok(json!({
        "api_keys": api_keys
    }))
}