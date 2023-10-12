use sqlx::{PgPool};
use sqlx::types::JsonValue;
use crate::errors::DatabaseError;
use crate::models::job_model::Job;
use crate::repositories::handle_error;

pub async fn create(
    pool: &PgPool,
    job_type: &str,
    payload: &JsonValue,
    priority: &i16,
    max_retries: &i16,
    name: &Option<String>,
    retry_after_minutes: &i16,
) -> Result<u64, DatabaseError> {
    match sqlx::query_as!(
        Job,
        "INSERT INTO jobs (job_type, payload, priority, max_retries, name, retry_after_minutes) VALUES ($1, $2, $3, $4, $5, $6)",
        &job_type,
        &payload,
        &priority,
        &max_retries,
        name.as_ref(),
        retry_after_minutes
    ).execute(pool).await {
        Ok(r) => Ok(r.rows_affected()),
        Err(e) => Err(handle_error(e)),
    }
}

pub async fn update(pool: &PgPool, job: &Job) -> Result<u64, DatabaseError> {
    match sqlx::query!(
        r#"
        UPDATE jobs
        SET status=$2, retries=$3, error_message=$4, started_at=$5, completed_at=$6, updated_at = NOW(), ready_after=$7
        WHERE id = $1
        "#,
        &job.id,
        &job.status,
        &job.retries,
        job.error_message.as_ref(),
        job.started_at.as_ref(),
        job.completed_at.as_ref(),
        job.ready_after.as_ref()
    ).execute(pool).await {
        Ok(r) => Ok(r.rows_affected()),
        Err(e) => Err(handle_error(e)),
    }
}

