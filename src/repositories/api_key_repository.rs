use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::errors::DatabaseError;
use crate::errors::DatabaseError::FailedToDelete;
use crate::models::api_key::ApiKey;
use crate::repositories::handle_error;

pub async fn key_exist(conn: &PgPool, needle: &str) -> Result<bool, DatabaseError> {
    match sqlx::query!("SELECT EXISTS(SELECT 1 FROM api_keys WHERE key = $1)", needle).fetch_one(conn).await {
        Ok(r) => Ok(r.exists.unwrap()),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn find_by_user_id(conn: &PgPool, user_id: &Uuid) -> Result<Vec<ApiKey>, DatabaseError> {
    match sqlx::query_as!(ApiKey, "SELECT * FROM api_keys WHERE user_id = $1", user_id).fetch_all(conn).await {
        Ok(result) => Ok(result),
        Err(e) => Err(handle_error(e))
    }
}

pub async fn delete(conn: &PgPool, id: &Uuid, user_id: &Uuid) -> Result<(), DatabaseError> {
    match sqlx::query_as!(ApiKey, "DELETE FROM api_keys WHERE id = $1 AND user_id = $2", id, user_id).execute(conn).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                return Ok(());
            }

            Err(FailedToDelete)
        },
        Err(e) => Err(handle_error(e))
    }
}

pub async fn create(pool: &PgPool, name: &str, key: &str, user_id: &Uuid) -> Result<ApiKey, DatabaseError> {
    match sqlx::query_as!(
        ApiKey,
        "INSERT INTO api_keys (name, key, user_id) VALUES ($1, $2, $3) RETURNING *",
        name,
        key,
        user_id
    ).fetch_one(pool).await {
        Ok(r) => Ok(r),
        Err(e) => Err(handle_error(e)),
    }
}