use sqlx::{Error};
use crate::errors::DatabaseError;
use crate::errors::DatabaseError::{RecordNotFound, DuplicateRecord, Every};

pub mod user_repository;
pub mod job_repository;

pub fn handle_error(error: Error) -> DatabaseError {
    return match error {
        Error::RowNotFound => RecordNotFound,
        Error::Database(ref err) => {
            if err.code().as_deref() == Some("23505") {
                return DuplicateRecord(error)
            }
            return Every(error)
        },
        _ => Every(error),
    }
}