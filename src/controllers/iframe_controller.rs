use actix_web::{http, HttpResponse, Responder, web, web::Data};
use sqlx::PgPool;
use crate::{utilities};
use crate::models::transaction::Transaction;
use crate::services::json_response::JsonResponse;

#[derive(serde::Deserialize)]
pub struct QueryParams {
    api_key: String,
    name: String,
    lang: Option<String>,
    accepts: Option<String>,
    default_accepts: Option<String>,
    collect_address: Option<bool>,
    collect_signature: Option<bool>,
}

const CC: &str = "cc";
const ACH: &str = "ach";

const PAYMENT_METHODS: [&str; 2] = [&CC, &ACH];


pub async fn load(query_params: web::Query<QueryParams>, conn: Data<PgPool>) -> impl Responder {
    let error_msg = Some(String::from("Invalid api token"));
    let mut error = &error_msg;

    // check if api key is valid
    /*if repo::query(move || repo::user_repository::get_by_api_key(&db, &api_key)).await.is_ok() {
        error = &None;
    }*/

    if error.is_none() {
        // @todo: check if user plan
    }

    if error.is_none() {
        // @todo: check if user has specified payment (name)
    }


    let accepts = query_params.accepts.as_deref().unwrap_or("all");
    let payment_methods = accepts.split(",").map(|a| a.trim()).collect::<Vec<&str>>();
    let default_accepts = query_params.default_accepts.as_deref().unwrap_or(CC);

    if !payment_methods.iter().all(|m| PAYMENT_METHODS.contains(m) || *m == "all") {
        error = &error_msg;
    }

    if !PAYMENT_METHODS.contains(&default_accepts) {
        error = &error_msg;
    }

    let mut tpl = utilities::templates::Template::new("views/iframe/loader.html")
        .add("error", &error.as_deref().unwrap_or(""))
        .add("accepts", &payment_methods)
        .add("collect_address", &query_params.collect_address.unwrap_or(false))
        .add("collect_signature", &query_params.collect_signature.unwrap_or(false))
        .add("default_accepts", &default_accepts);

    HttpResponse::Ok()
        .append_header((http::header::X_FRAME_OPTIONS, "allowall"))
        .append_header(http::header::ContentType(mime::TEXT_HTML_UTF_8))
        .body(tpl.render())
}

pub async fn submit(form_data: web::Json<Transaction>, _: Data<PgPool>) -> impl Responder {
    JsonResponse::new().error("hello")


    // Response::new().ok(json!(form_data))
}