use argon2::Result;
use axum::{
    http::{Method, Uri},
    Extension,
};
use serde::Serialize;
use serde_json::{json, to_value, Value};
use serde_with::skip_serializing_none;
use time::OffsetDateTime;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

use crate::{error::ClientError, models::user::User, Error};

pub fn init() {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::trace!("output enabled");
    tracing::debug!("output enabled");
    tracing::info!("output enabled");
    tracing::warn!("output enabled");
    tracing::error!("output enabled");

    println!();
}

pub fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    user: Option<Extension<User>>,
    service_error: Option<&Error>,
    status_code: String,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = OffsetDateTime::now_utc();

    let service_error_type = service_error.map(|e| e.as_ref().to_owned());
    let service_error_data = to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        log_id: uuid.to_string(),
        timestamp: timestamp.to_string(),

        user_id: user.map(|u| u.id),

        http_path: uri.to_string(),
        http_method: req_method.to_string(),

        client_error: client_error.map(|e| e.as_ref().to_string()),
        status_code,

        service_error_type,
        service_error_data,
    };

    tracing::debug!(
        "{:-<15} |> \n{} |> {}\n",
        "REQUEST_LOG ",
        json!(log_line),
        file!()
    );

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    log_id: String,
    timestamp: String,

    user_id: Option<Uuid>,

    http_path: String,
    http_method: String,

    client_error: Option<String>,
    status_code: String,

    service_error_type: Option<String>,
    service_error_data: Option<Value>,
}
