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
use sqlx::Row;

use crate::{
    AppState,
    middlewares::jwt::JwtMiddleware,
    models::jwt::TokenClaims,
    models::response::Response,
    models::users::{LoginUserSchema, RegisterUserSchema, User},
};

pub fn get_cookie(value: String, expire_at: i64) -> Cookie<'static> {
    Cookie::build("token", value)
        .path("/")
        .max_age(ActixWebDuration::new(expire_at, 0))
        .http_only(true)
        .finish()
}

pub async fn register(body: Json<RegisterUserSchema>, data: web::Data<AppState>) -> impl Responder {
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(body.email.to_owned())
        .fetch_one(&data.db)
        .await
        .unwrap()
        .get(0);

    if exists {
        return Response::new().status(SC::CONFLICT).error("A user with email already exist in our system");
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (first_name,last_name,email,password) VALUES ($1, $2, $3, $4) RETURNING *",
        body.first_name.to_string(),
        body.last_name.to_string(),
        body.email.to_string().to_lowercase(),
        hashed_password
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(user) => {
            let token = create_jwt_token(&data, &user);
            Response::new().cookie(get_cookie(token.to_string(), 60 * 60)).ok(user.get_filtered())
        }
        Err(e) => {
            Response::new().error(format!("{:?}", e).as_str())
        }
    }
}

pub async fn login(body: Json<LoginUserSchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", body.email)
        .fetch_optional(&data.db)
        .await
        .unwrap();

    let is_valid = query_result.to_owned().map_or(false, |user| {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return Response::new().status(SC::UNAUTHORIZED).error("Invalid email or password");
    }

    let user = query_result.unwrap();
    let token = create_jwt_token(&data, &user);

    Response::new().cookie(get_cookie(token.to_string(), 60 * 60)).ok(user.get_filtered())
}

fn create_jwt_token(data: &Data<AppState>, user: &User) -> String {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    ).unwrap()
}

pub async fn logout(_: JwtMiddleware) -> impl Responder {
    /* let cookie = Cookie::build("token", "")
         .path("/")
         .max_age(ActixWebDuration::new(-1, 0))
         .http_only(true)
         .finish();*/

    Response::new().cookie(get_cookie("".to_string(), -1)).ok("Logged Out")
}

