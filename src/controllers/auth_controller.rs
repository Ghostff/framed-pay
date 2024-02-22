use actix_web::{
    http::StatusCode as SC, Responder,
};
use actix_web::web::Data;
use actix_web_validator::Json;
use chrono::{ Utc, Duration };
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::{PgPool};

use crate::{models::jwt::TokenClaims, services::json_response::JsonResponse, models::user::{LoginUserSchema, RegisterUserSchema, User}, repositories as repo, services::email::Email, services::cookie::{AccessTokenCookie, Cookie}, services::job_service::Job, services, utilities, check_bot, check_duplicate_email};
use crate::config::{ENV};
use crate::errors::DatabaseError;
use crate::models::user::{PasswordChangeSchema, ResetUserPasswordSchema, VerifyPasswordResetTokenSchema};

pub async fn register(body: Json<RegisterUserSchema>, conn: Data<PgPool>) -> impl Responder {
    check_bot!(&body.confirm_email, &body.recaptcha_token);
    let email = body.email.to_lowercase();
    let conflict_msg = "Email already exist in our system";

    let exist = match repo::user_repository::email_exist(&conn, &email).await {
        Err(e) => return JsonResponse::fetal(e),
        Ok(r) => r,
    };

    if exist {
        return JsonResponse::new().status(SC::CONFLICT).error(&conflict_msg);
    }

    let password = utilities::crypto::hash_password(&body.password);
    let user = match repo::user_repository::create(
        &conn,
        &body.first_name,
        &body.last_name,
        &email,
        &password,
    ).await {
        Err(DatabaseError::DuplicateRecord(_)) => return JsonResponse::new().status(SC::CONFLICT).error(conflict_msg),
        Err(error) => return JsonResponse::fetal(error),
        Ok(u) => u,
    };

    match services::user_service::attach_api_key(&conn, &user, "Default").await {
        Err(e) => return JsonResponse::fetal(e),
        Ok(_) => {}
    }

     JsonResponse::new()
         .cookie(AccessTokenCookie::set(create_jwt_token(&user).as_ref()))
         .ok(user.get_filtered())
}

pub async fn login(body: Json<LoginUserSchema>, conn: Data<PgPool>) -> impl Responder {
    check_bot!(&body.confirm_email, &body.recaptcha_token);
    let email = body.email.to_lowercase();
    let mut user  = match repo::user_repository::find_by_email(&conn, &email).await {
        Err(DatabaseError::RecordNotFound) => return JsonResponse::unauthorized("Invalid email or password"),
        Err(e) => return JsonResponse::fetal(e),
        Ok(u) => u,
    };

    if !utilities::crypto::verify_password(&user.password, &body.password) {
        return JsonResponse::unauthorized("Invalid email or password");
    }

    user.last_logged_in_at = Some(Utc::now());
    if let Err(e) = repo::user_repository::update(&conn, &user).await {
        return JsonResponse::fetal(e)
    }

    JsonResponse::new()
        .cookie(AccessTokenCookie::set(create_jwt_token(&user).as_str()))
        .ok(user.get_filtered())
}

pub async fn logout(_: User) -> impl Responder {
    JsonResponse::new().cookie(AccessTokenCookie::remove()).message_key("message").ok("Logged Out")
}

pub async fn password_reset(body: Json<ResetUserPasswordSchema>, conn: Data<PgPool>, job: Data<Job>) -> impl Responder {
    let email = body.email.to_lowercase();
    let mut user = match repo::user_repository::find_by_email(&conn, &email).await {
        Err(DatabaseError::RecordNotFound) => return JsonResponse::unauthorized("Success"),
        Err(e) => return JsonResponse::fetal(e),
        Ok(u) => u
    };

    let reset_token = utilities::str::get_random(200);
    let url = format!("{}/password-reset/{reset_token}/{}", ENV.app_url, user.id);

    let mut template = utilities::templates::Template::new("emails/password_reset.html")
        .add("name", &user.last_name)
        .add("url", &url);

    user.password_reset_token = Some(format!("{reset_token}|{}", Utc::now().to_rfc3339()));
    if let Err(e) = repo::user_repository::update(&conn, &user).await {
        return JsonResponse::fetal(e)
    }

    let email = Email::html(body.email.clone(), "Password Reset", template.render());
    match job.queue(email).await {
        Err(e) => return JsonResponse::fetal(e),
        _ => {}
    };

    JsonResponse::new().ok("Success")
}

pub async fn verify_password_reset_token(body: Json<VerifyPasswordResetTokenSchema>, conn: Data<PgPool>) -> impl Responder {
    match services::user_service::get_user_by_token(&conn, &body).await {
        Ok(_) => JsonResponse::success(),
        Err(_) => JsonResponse::new().status(SC::UNAUTHORIZED).error("Invalid or expired token.")
    }
}

pub async fn change_password(body: Json<PasswordChangeSchema>, conn: Data<PgPool>) -> impl Responder {
    let token = VerifyPasswordResetTokenSchema {
        token: body.token.clone(),
        uid: body.uid.clone(),
    };

    let mut user = match services::user_service::get_user_by_token(&conn, &token).await {
        Ok(u) => u,
        Err(_) => return JsonResponse::new().status(SC::UNAUTHORIZED).error("Invalid or expired token."),
    };

    user.password_reset_token = None;
    user.password = utilities::crypto::hash_password(&body.password);

    if let Err(e) = repo::user_repository::update(&conn, &user).await {
        return JsonResponse::fetal(e)
    }

    JsonResponse::success()
}

fn create_jwt_token(user: &User) -> String {
    let now = Utc::now();
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp: (now + Duration::minutes(60)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(ENV.jwt_secret.as_ref())).unwrap()
}


