use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use validator::Validate;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApiKey {
    pub id: uuid::Uuid,
    pub name: String,
    pub key: String,
    pub user_id: uuid::Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

lazy_static! {
    static ref NAME: Regex = Regex::new(r"^[\w\-\s]{2,200}$").unwrap();
}
#[derive(Debug, Deserialize, Validate)]
pub struct CreateApiKeySchema {
    #[validate(regex(path = "NAME", message = "Name must be alphanumeric only and must be between 2 to 6 characters"))]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct FilteredApiKey {
    pub id: String,
    pub name: String,
    pub key: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ApiKey {
    pub fn get_filtered(&self) -> FilteredApiKey {
        FilteredApiKey {
            id: self.id.to_string(),
            name: self.name.to_owned(),
            key: self.key.to_owned(),
            created_at: self.created_at.unwrap(),
            updated_at: self.updated_at.unwrap(),
        }
    }
}