
#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error("Token not found")]
    TokenNotFound,
    #[error("Invalid token format")]
    InvalidTokenFormat,
    #[error("Invalid date time format")]
    InvalidDateTimeFormat,
    #[error("Token expired")]
    TokenExpired,
    #[error("Could not fetch token")]
    Error,
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Invalid UUID")]
    InvalidUuid,
    #[error("No record found")]
    RecordNotFound,
    #[error("Duplicate data error: {0}")]
    DuplicateRecord(sqlx::Error),
    #[error("Database error: {0}")]
    Every(sqlx::Error),
}

#[derive(Debug)]
pub enum QueueError {
    Error(String),
}



