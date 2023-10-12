use std::collections::HashMap;
use actix_web::dev::ServiceResponse;
use actix_web::{Error, error, HttpRequest, HttpResponse};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlerResponse;
use reqwest::header;

pub fn json_validation(err: actix_web_validator::Error, _: &HttpRequest) -> Error {
    use actix_web_validator::Error;
    use crate::services::json_response::JsonResponse;

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

            res = JsonResponse::new()
                .status(StatusCode::BAD_REQUEST)
                .payload_key("errors")
                .error(&errors);
        }
        Error::JsonPayloadError(e) => {
            let message = match &e {
                error::JsonPayloadError::Deserialize(er) => er.to_string().replace("`", "\""),
                _ => "Invalid JSON payload".to_string()
            };
            res = JsonResponse::new().status(StatusCode::BAD_REQUEST).error(message);
        }
        _ => {
            res = JsonResponse::new().status(StatusCode::BAD_REQUEST).error("Could not process request");
        }
    }

    InternalError::from_response("bad Request", res).into()
}

pub fn web<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, Error> {
    // This function is only expected to be called for errors, so if there is no error, return
    if res.response().error().is_none() || res.response().status().as_u16() < 500 {
        return Ok(ErrorHandlerResponse::Response(res.map_into_left_body()));
    }

    let (req, mut res) = res.into_parts();
    // @todo content content-encoding: "gzip" should be fixed an allowed
    res.headers_mut().remove(header::CONTENT_ENCODING);
    let mut response = HttpResponse::build(res.status());

    // Copy headers from the existing response to the new response
    for (header_name, header_value) in res.headers().iter() {
        response.append_header(( header_name.clone(), header_value.clone()));
    }

    let mut body = "Internal Server Error";
    if let Some(content_type) = req.headers().get(header::CONTENT_TYPE) {
        if content_type == mime::APPLICATION_JSON.as_ref() {
            body = r#"{"error": "Internal Server Error"}"#;
        }
    }

    let response = ServiceResponse::new(req, response.body(body));
    Ok(ErrorHandlerResponse::Response(response.map_into_right_body()))
}