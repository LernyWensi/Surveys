use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct SurveyResult {
    pub id: Uuid,
    pub survey_id: Uuid,
    pub result_data: Value,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl SurveyResult {
    pub fn filter_for_response(&self) -> ResponseSurveyResult {
        ResponseSurveyResult {
            id: self.id,
            survey_id: self.survey_id.to_owned(),
            result_data: self.result_data.to_owned(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResponseSurveyResult {
    pub id: Uuid,
    pub survey_id: Uuid,
    pub result_data: Value,
}

#[derive(Debug)]
pub struct CreateSurveyResult {
    pub survey_id: Uuid,
    pub result_data: Value,
}

#[derive(Debug, Deserialize)]
pub struct RequestCreateSurveyResult {
    pub survey_id: Uuid,
    pub result_data: Value,
}
