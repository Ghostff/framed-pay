use actix_web::{HttpMessage, HttpRequest, Responder, web};

use crate::AppState;
use crate::controllers::auth::get_cookie;
use crate::middlewares::jwt::JwtMiddleware;
use crate::models::response::Response;
use crate::models::users::User;

pub async fn me(req: HttpRequest, data: web::Data<AppState>, jwt: JwtMiddleware) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id).fetch_one(&data.db).await {
        Ok(user) => Response::new().ok(user.get_filtered()),
        Err(err) => {
            Response::new().cookie(get_cookie("".to_string(), -1)).ok("Logged Out")
        }
    }
}

// make auth guard
// fix password reset