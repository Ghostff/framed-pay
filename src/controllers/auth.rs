use actix_web::{
    cookie::{Cookie, time::Duration as ActixWebDuration},
    http::StatusCode as SC, Responder,
    web,
};
use actix_web::web::Data;
use actix_web_validator::Json;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, rand_core::OsRng, SaltString},
};
use chrono::{Duration, prelude::*};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{
    AppState,
    models::jwt::TokenClaims,
    models::response::Response,
    models::users::{LoginUserSchema, RegisterUserSchema, User},
    repositories as repo
};
use crate::config::Config;
use crate::models::users::ResetUserPasswordSchema;

pub fn get_cookie(value: String, expire_at: i64) -> Cookie<'static> {
    Cookie::build("token", value)
        .path("/")
        .max_age(ActixWebDuration::new(expire_at, 0))
        .http_only(true)
        .secure(false) // @todo: remove on prod
        .finish()
}

pub async fn register(body: Json<RegisterUserSchema>, data: Data<AppState>) -> impl Responder {
    let db = data.db.clone();
    let email = body.email.clone();
    let exists = web::block(move || repo::user::email_exist(&db, email.as_str()))
        .await
        .unwrap_or(false);

    if exists {
        return Response::new().status(SC::CONFLICT).error("A user with email already exist in our system");
    }

    let db = data.db.clone();
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let insert_query = web::block(move || repo::user::create_and_get(
        &db,
        body.first_name.as_str(),
        body.last_name.as_str(),
        body.email.to_string().to_lowercase().as_str(),
        hashed_password.as_str()
    )).await;

    match insert_query {
        Ok(Ok(user)) => {
            let token = create_jwt_token(&data.env.clone(), &user);
            Response::new().cookie(get_cookie(token.to_string(), 60 * 60)).ok(user.get_filtered())
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Response::new().error("Error occurred")
        }
        _ => {
            Response::new().error("Error occurred")
        }
    }
}

pub async fn login(body: Json<LoginUserSchema>, data: Data<AppState>) -> impl Responder {
    let db = data.db.clone();
    let email = body.email.clone();
    let query_result = web::block(move || repo::user::get_by_email(&db, email.as_str()))
        .await;

    let is_valid = query_result.as_ref().map_or_else(|e| {
        eprintln!("Error: {:?}", e);
        false
    }, |user| {
        let parsed_hash = PasswordHash::new(user.as_ref().unwrap().password.as_str()).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .is_ok()
    });

    if !is_valid {
        return Response::new().status(SC::UNAUTHORIZED).error("Invalid email or password");
    }

    let user = query_result.unwrap().unwrap();
    let token = create_jwt_token(&data.env, &user);

    Response::new().cookie(get_cookie(token.to_string(), 60 * 60)).ok(user.get_filtered())
}

pub async fn logout(_: User) -> impl Responder {
    Response::new().cookie(get_cookie("".to_string(), -1)).ok("Logged Out")
}

pub async fn reset_password(body: Json<ResetUserPasswordSchema>, data: Data<AppState>) -> impl Responder {
    let db = data.db.clone();
    let email = body.email.clone();
    let user_exist = web::block(move || repo::user::email_exist(&db, email.as_str()))
        .await
        .unwrap_or(false);

    if user_exist {

    }

    Response::new().ok("Success")
}

fn create_jwt_token(env: &Config, user: &User) -> String {
    let now = Utc::now();
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp: (now + Duration::minutes(60)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(env.jwt_secret.as_ref())).unwrap()
}


