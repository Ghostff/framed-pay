use std::collections::HashMap;
use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, REFERER};
use serde::{Serialize};
use serde_json::{Value};
use crate::config::ENV;

lazy_static! {
    static ref RECAPTCHA_URL: String = format!(
        "https://recaptchaenterprise.googleapis.com/v1/projects/{}/assessments?key={}",
        &ENV.app_name,
        &ENV.google_api_key
    );
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct RecaptchaEvent<'a> {
    token: &'a str,
    siteKey: &'a str,
    expectedAction: &'a str,
}

#[derive(Serialize)]
struct RecaptchaRequestBody<'a> {
    event: RecaptchaEvent<'a>,
}
pub async fn is_valid(honeypot: &str, recaptcha_token: &str) -> bool {
    if honeypot != "" {
        return false
    }

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(REFERER, ENV.app_url.parse().unwrap());

    let res = match client.post(&*RECAPTCHA_URL)
        .headers(headers)
        .json(&RecaptchaRequestBody {
            event: RecaptchaEvent {
                token: recaptcha_token,
                siteKey: &ENV.google_recaptcha_key,
                expectedAction: "FORM_SUBMIT",
            }
        })
        .send()
        .await {
        Ok(r) => r,
        Err(e) => {
            log::error!("{:?}", e);
            return false;
        }
    };

    match res.json::<HashMap<String, Value>>().await {
        Ok(json) => {
            if !json.contains_key("error")
                && json.contains_key("tokenProperties")
                && json.get("tokenProperties").unwrap().get("valid").unwrap() == true
            {
                return true;
            }

            log::error!("invalid recaptcha token:\n{:?}", json);
        }
        Err(e) => {
            log::error!("{:?}", e);
        }
    }

    return false;
}

#[macro_export]
macro_rules! check_bot {
    ($honeypot:expr, $recaptcha_token:expr) => {
        if !crate::services::request_validator::is_valid($honeypot, $recaptcha_token).await {
            return JsonResponse::success();
        }
    };
}