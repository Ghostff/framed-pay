use std::fmt::Debug;

use actix_web::{ HttpResponse};
use actix_web::cookie::Cookie;
use actix_web::http::StatusCode;
use serde::Serialize;
use serde_json::{json, Value};

pub struct JsonResponse<'a> {
    status: StatusCode,
    message_key: &'a str,
    payload_key: &'a str,
    cookie: Option<Cookie<'a>>,
}

impl<'a> JsonResponse<'a> {
    pub fn new() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message_key: "error",
            payload_key: "data",
            cookie: None,
        }
    }

    pub fn status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status;
        self
    }

    #[allow(dead_code)]
    pub fn message_key(&mut self, name: &'a str) -> &mut Self {
        self.message_key = name;
        self
    }

    pub fn payload_key(&mut self, name: &'a str) -> &mut Self {
        self.payload_key = name;
        self
    }

    pub fn cookie(&mut self, cookie: Cookie<'a>) -> &mut Self {
        self.cookie = Option::from(cookie);
        self
    }

    pub fn json_body(&self, message: impl Serialize, name: &str) -> Value {
        let status_code = self.status.as_u16();
        let message_value: Value = json!(message);

        let json_response = if let Value::String(s) = &message_value {
            json!({ "status_code": status_code, name: s })
        } else {
            json!({ "status_code": status_code, self.payload_key: message_value })
        };

        json_response
    }

    // Internal server error.
    pub fn fetal<T: Debug>(e: T) -> HttpResponse {
        log::error!("{:?}", e);
        return Self::new().build("Internal server error")
    }

    pub fn unauthorized(message: &str) -> HttpResponse  {
        return Self::new().status(StatusCode::UNAUTHORIZED).error(message);
    }

    pub fn bad_request(name: &str, message: &str) -> HttpResponse  {
        return Self::new()
            .status(StatusCode::BAD_REQUEST)
            .payload_key("errors")
            .build(json!({name: vec![message]}));
    }

    pub fn success() -> HttpResponse {
        return Self::new().message_key("message").ok("Success")
    }

    pub fn error(&mut self, message: impl Serialize) -> HttpResponse {
        self.build(message)
    }

    pub fn ok(&mut self, message: impl Serialize) -> HttpResponse {
        self.status(StatusCode::OK).build(message)
    }

    fn build(&mut self, message: impl Serialize) -> HttpResponse {
        let mut response = HttpResponse::build(self.status);
        if let Some(cookie) = &self.cookie {
            response.cookie(cookie.clone());
        }

        response.json(self.json_body(message, self.message_key))
    }
}