use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub avatar: String,
    pub last_logged_in_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserSchema {
    #[validate(length(min = 1, max = 100, message = "First name must be between 1 to 100"))]
    pub first_name: String,
    #[validate(length(min = 1, max = 100, message = "Last name must be between 1 to 100"))]
    pub last_name: String,
    #[validate(length(min = 4, max = 250, message = "Email must be between 4 to 250"))]
    #[validate(email(code = "code_str", message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be greater than 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUserSchema {
    #[validate(length(min = 4, max = 250, message = "Email must be between 4 to 250"))]
    #[validate(email(code = "code_str", message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be greater than 8 characters"))]
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn get_filtered(self) -> FilteredUser {
        FilteredUser {
            id: self.id.to_string(),
            email: self.email.to_owned(),
            first_name: self.first_name.to_owned(),
            last_name: self.last_name.to_owned(),
            avatar: self.avatar.to_owned(),
            role: self.role.to_owned(),
            created_at: self.created_at.unwrap(),
            updated_at: self.updated_at.unwrap(),
        }
    }
}