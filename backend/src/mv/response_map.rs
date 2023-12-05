use axum::{
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde_json::to_value;
use uuid::Uuid;

use crate::{
    logger::log_request,
    models::{user::User, DefaultResponse, ResponseError},
};

pub async fn response_map(
    user: Option<Extension<User>>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    tracing::debug!("{:-<15} |> response_map |> {}", "MIDDLEWARE ", file!());

    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<crate::Error>();
    let client_error_and_status = service_error.map(|e| e.client_error_and_status());

    let error_response = client_error_and_status
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_message = client_error.as_ref().to_owned();
            let client_error_detail = to_value(client_error)
                .ok()
                .as_ref()
                .and_then(|v| v.get("detail"))
                .map(|y| y.to_owned());

            let client_error_body = DefaultResponse {
                success: false,
                message: client_error_message,
                data: None,
                error: Some(ResponseError {
                    request_id: uuid,
                    detail: client_error_detail,
                }),
            };

            (*status_code, Json(client_error_body)).into_response()
        });

    let (status_code, client_error) = client_error_and_status.unzip();
    let status_code = status_code.unwrap_or(res.status()).to_string();

    let _ = log_request(
        uuid,
        req_method,
        uri,
        user,
        service_error,
        status_code,
        client_error,
    );

    error_response.unwrap_or(res)
}
