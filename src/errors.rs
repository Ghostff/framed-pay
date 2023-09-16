
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
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Invalid UUID")]
    InvalidUuid,
    #[error("Database error: {0}")]
    DatabaseConnectionFailed(String),
}

#[derive(Debug)]
pub enum FailSafeError<'a> {
    Error(&'a str),
}



