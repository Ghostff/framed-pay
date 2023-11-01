use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Job {
    pub id: uuid::Uuid,
    pub job_type: String,
    pub name: Option<String>,
    pub payload: JsonValue,
    pub status: String,
    pub priority: i16,
    pub retries: i16,
    pub max_retries: i16,
    pub error_message: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub retry_after_minutes: i16,
    pub ready_after: Option<DateTime<Utc>>,
}

pub struct JobStatus;

impl JobStatus {
    pub const QUEUED: &'static str = "queued";
    // pub const PROCESSING: &'static str = "processing";
    pub const COMPLETED: &'static str = "completed";
    pub const FAILED: &'static str = "failed";
}