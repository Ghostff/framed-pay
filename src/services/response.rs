use std::collections::HashMap;

use actix_web::{error, HttpRequest, HttpResponse};
use actix_web::cookie::Cookie;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web_validator::Error;
use actix_web_validator::Error::JsonPayloadError;
use serde::Serialize;
use serde_json::{json, Value};

pub struct Response<'a> {
    status: StatusCode,
    message_key: &'a str,
    payload_key: &'a str,
    cookie: Option<Cookie<'a>>,
}

impl<'a> Response<'a> {
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
    pub fn fetal(&mut self) -> HttpResponse {
        self.build("Internal server error")
    }

    pub fn success(&mut self) -> HttpResponse {
        self.ok("Success")
    }

    pub fn error(&mut self, message: impl Serialize) -> HttpResponse {
        self.build(message)
    }

    pub fn ok(&mut self, message: impl Serialize) -> HttpResponse {
        self.status(StatusCode::OK).build(message)
    }

    pub fn error_handler(err: Error, _: &HttpRequest) -> actix_web::Error {
        let res: HttpResponse;

        match err {
            Error::Validate(error) => {
                let mut errors: HashMap<String, Vec<String>> = HashMap::new();

                for (key, error) in error.field_errors() {
                    for field in error {
                        if let Some(message) = &field.message {
                            errors.insert(key.to_string(), vec![message.to_string()]);
                            break; // On error at a time.
                        }
                    }
                }

                res = Self::new()
                    .status(StatusCode::BAD_REQUEST)
                    .payload_key("errors")
                    .error(&errors);
            }
            JsonPayloadError(e) => {
                let message = match &e {
                    error::JsonPayloadError::Deserialize(er) => er.to_string().replace("`", "\""),
                    _ => "Invalid JSON payload".to_string()
                };

                res = Self::new().error(message);
            }
            _ => {
                res = Self::new().error("Could not process request");
            }
        }

        InternalError::from_response("validation error", res).into()
    }

    fn build(&mut self, message: impl Serialize) -> HttpResponse {
        let mut response = HttpResponse::build(self.status);
        if let Some(cookie) = &self.cookie {
            response.cookie(cookie.clone());
        }

        response.json(self.json_body(message, self.message_key))
    }
}