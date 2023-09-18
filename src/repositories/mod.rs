use diesel::QueryResult;
use diesel::result::Error;
use crate::errors::FailSafeError;

pub mod user_repository;

pub async fn query<'a, T, F>(callback: F) -> Result<T, FailSafeError<'a>>
    where
        F: FnOnce() -> QueryResult<T> + Send + 'static,
        T: Send + 'static,
{
    use actix_web::web;

    match web::block(callback).await {
        Ok(result) => {
            match result {
                Ok(t) => return Ok(t),
                Err(Error::NotFound) => return Err(FailSafeError::Error("Not found")),
                Err(failsafe_error) => {
                    log::error!("{:?}", failsafe_error);
                },
            }
        },
        Err(blocking_error) => {
            log::error!("{:?}", blocking_error);
        },
    }

    Err(FailSafeError::Error("Internal Server error"))
}
