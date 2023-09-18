use actix_web::{
    cookie::{Cookie, time::Duration as ActixWebDuration},
    http::StatusCode as SC, Responder,
};
use actix_web::web::Data;
use actix_web_validator::Json;
use apalis::redis::RedisStorage;
use argon2::{Argon2, password_hash::{PasswordHash, PasswordVerifier}};
use chrono::{ Utc, Duration };
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{DBPool, models::jwt_model::TokenClaims, services::response::Response, models::user_model::{LoginUserSchema, RegisterUserSchema, User}, repositories as repo, services};
use crate::config::{ENV};
use crate::models::user_model::{PasswordChangeSchema, ResetUserPasswordSchema, VerifyPasswordResetTokenSchema};
use crate::services::email::Email;

pub fn get_cookie(value: String, expire_at: i64) -> Cookie<'static> {
    Cookie::build("token", value)
        .path("/")
        .max_age(ActixWebDuration::new(expire_at, 0))
        .http_only(true)
        .secure(!cfg!(debug_assertions)) // @todo: remove on prod
        .finish()
}

pub async fn register(body: Json<RegisterUserSchema>, conn: Data<DBPool>) -> impl Responder {
    let db = conn.clone();
    let email = body.email.clone();

    if repo::query(move || repo::user_repository::get_by_email(&db, email.as_str())).await.is_ok() {
        return Response::new().status(SC::CONFLICT).error("A user with email already exist in our system");
    }

    let mut api_key;
    loop {
        api_key = services::str::get_random(100);
        let key = api_key.clone();
        let db = conn.clone();
        if repo::query(move || repo::user_repository::get_by_api_key(&db, &key)).await.is_err() {
            break;
        }
    }

    let db = conn.clone();
    let insert_query = repo::query(move || repo::user_repository::create_and_get(
        &db,
        body.first_name.as_str(),
        body.last_name.as_str(),
        body.email.to_string().to_lowercase().as_str(),
        services::str::hash(&body.password).as_str(),
        &api_key
    )).await;

    if insert_query.is_err() {
        Response::new().fetal();
    }

    let user = insert_query.unwrap();
    let token = create_jwt_token(&user);
    Response::new().cookie(get_cookie(token.to_string(), 60 * 60)).ok(user.get_filtered())
}

pub async fn login(body: Json<LoginUserSchema>, conn: Data<DBPool>) -> impl Responder {
    let db = conn.clone();
    let email = body.email.to_string();
    let query_result = repo::query(move || repo::user_repository::get_by_email(&db, &email)).await;

    if query_result.is_err() {
        Response::new().fetal();
    }

    let mut user = query_result.unwrap();
    let parsed_hash = PasswordHash::new(&user.password.as_str()).unwrap();
    if !Argon2::default().verify_password(body.password.as_bytes(), &parsed_hash).is_ok() {
        return Response::new().status(SC::UNAUTHORIZED).error("Invalid email or password");
    }

    let db = conn.clone();
    user.last_logged_in_at = Some(Utc::now());
    let tmp_user = user.clone();
    let _ = repo::query(move || repo::user_repository::update(&db, &tmp_user)).await;

    Response::new().cookie(get_cookie(create_jwt_token(&user), 60 * 60)).ok(user.get_filtered())
}

pub async fn logout(_: User) -> impl Responder {
    Response::new().cookie(get_cookie("".to_string(), -1)).ok("Logged Out")
}

pub async fn password_reset(body: Json<ResetUserPasswordSchema>, conn: Data<DBPool>, queue: Data<RedisStorage<Email>>) -> impl Responder {
    let db = conn.clone();
    let email = body.email.to_string();
    let user_exist = repo::query(move || repo::user_repository::get_by_email(&db, &email)).await;

    if user_exist.is_ok() {
        let mut user = user_exist.unwrap();
        let reset_token = services::str::get_random(200);
        let url = format!("{}/password-reset/{reset_token}/{}", ENV.app_url.clone(), user.id);
        let name = user.last_name.clone();

        let mut template = services::templates::Template::new("emails/password_reset.html")
            .add("name", name.as_str())
            .add("url", url.as_str());

        let db = conn.clone();
        user.password_reset_token = Some(format!("{reset_token}|{}", Utc::now().to_rfc3339()));
        if repo::query(move || repo::user_repository::update(&db, &user)).await.is_err() {
            return Response::new().error("Error resetting password.");
        }

        services::queue::dispatch(
            queue,
            Email::html( body.email.clone(), "Password Reset", template.render())
        ).await;
    }

    Response::new().ok("Success")
}

pub async fn verify_password_reset_token(body: Json<VerifyPasswordResetTokenSchema>, conn: Data<DBPool>) -> impl Responder {
    match services::user::get_user_by_token(&conn, &body).await {
        Ok(_) => Response::new().success(),
        Err(_) => Response::new().status(SC::UNAUTHORIZED).error("Invalid or expired token.")
    }
}

pub async fn change_password(body: Json<PasswordChangeSchema>, conn: Data<DBPool>) -> impl Responder {
    let token = VerifyPasswordResetTokenSchema {
        token: body.token.clone(),
        uid: body.uid.clone(),
    };

    let mut user = match services::user::get_user_by_token(&conn, &token).await {
        Ok(u) => u,
        Err(_) => return Response::new().status(SC::UNAUTHORIZED).error("Invalid or expired token."),
    };

    user.password_reset_token = None;
    user.password = services::str::hash(&body.password);

    if repo::query(move || repo::user_repository::update(&conn, &user)).await.is_err() {
        return Response::new().fetal();
    }

    Response::new().success()
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


