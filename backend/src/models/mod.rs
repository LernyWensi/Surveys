use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

pub mod survey;
pub mod survey_result;
pub mod user;

#[derive(Debug, Serialize)]
pub struct DefaultResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Value>,
    pub error: Option<ResponseError>,
}

#[derive(Debug, Serialize)]
pub struct ResponseError {
    pub request_id: Uuid,
    pub detail: Option<Value>,
}
