use actix_web::{ HttpRequest, Responder };

use crate::services::response::Response;
use crate::models::user_model::User;

pub async fn me(_: HttpRequest, user: User) -> impl Responder {
    Response::new().ok(user.get_filtered())
}

// make auth guard
// fix password reset