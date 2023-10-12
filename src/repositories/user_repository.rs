use sqlx::{Error, PgPool};
use uuid::Uuid;
use models::user_model::User;
use crate::{models};
use crate::errors::DatabaseError;
use crate::repositories::handle_error;


pub async fn email_exist(conn: &PgPool, needle: &str) -> Result<bool, Error> {
    match sqlx::query!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", needle).fetch_one(conn).await {
        Ok(r) => Ok(r.exists.unwrap()),
        Err(e) => Err(e),
    }
}

pub async fn api_key_exist(conn: &PgPool, needle: &str) -> Result<bool, Error> {
    match sqlx::query!("SELECT EXISTS(SELECT 1 FROM users WHERE api_key = $1)", needle).fetch_one(conn).await {
        Ok(r) => Ok(r.exists.unwrap()),
        Err(e) => Err(e),
    }
}

pub async fn find_by_email(conn: &PgPool, needle: &str) -> Result<User, DatabaseError> {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", needle).fetch_one(conn).await {
        Ok(result) => Ok(result),
        Err(e) => Err(handle_error(e))
    }
}

pub async fn find_by_password_reset_token(conn: &PgPool, uuid: &Uuid, needle: &str) -> Result<User, DatabaseError> {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 AND password_reset_token LIKE $2 || '|%'", uuid, needle).fetch_one(conn).await {
        Ok(result) => Ok(result),
        Err(e) => Err(handle_error(e))
    }
}

pub async fn find(conn: &PgPool, needle: &Uuid) -> Result<User, DatabaseError> {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", needle).fetch_one(conn).await {
        Ok(result) => Ok(result),
        Err(e) => Err(handle_error(e))
    }
}

pub async fn create(pool: &PgPool, f_name: &str, l_name: &str, email_address: &str, pass: &str, key: &str) -> Result<User, DatabaseError> {
    match sqlx::query_as!(
        User,
        "INSERT INTO users (first_name, last_name, email, password, api_key) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        f_name,
        l_name,
        email_address,
        pass,
        key
    ).fetch_one(pool).await {
        Ok(r) => Ok(r),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn update(pool: &PgPool, user: &User) -> Result<u64, DatabaseError> {
    match sqlx::query!(
        r#"
        UPDATE users
        SET first_name=$2, last_name=$3, email=$4, avatar=$5, password=$6, password_reset_token=$7, api_key=$8, role=$9, last_logged_in_at=$10, updated_at = NOW()
        WHERE id = $1
        "#,
        &user.id,
        &user.first_name,
        &user.last_name,
        &user.email,
        &user.avatar,
        &user.password,
        user.password_reset_token,
        &user.api_key,
        &user.role,
        user.last_logged_in_at,
    ).execute(pool).await {
        Ok(r) => Ok(r.rows_affected()),
        Err(e) => Err(handle_error(e)),
    }
}