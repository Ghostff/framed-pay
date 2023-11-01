use sqlx::{PgPool};

use crate::{utilities};
use crate::errors::DatabaseError;
use crate::repositories::api_key_repository;
pub async fn generate_key(conn: &PgPool) -> Result<String, DatabaseError> {
    let mut key;
    loop {
        key = utilities::str::get_random(100);
        match api_key_repository::key_exist(&conn, &key).await {
            Err(e) => return Err(e),
            Ok(r) => if r == false {
                return Ok(key);
            },
        };
    }
}
