use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Survey {
    pub id: Uuid,
    pub title: String,
    pub user_id: Uuid,
    pub survey_data: Value,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl Survey {
    pub fn filter_for_response(&self) -> ResponseSurvey {
        ResponseSurvey {
            id: self.id,
            title: self.title.to_owned(),
            survey_data: self.survey_data.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct CreateSurvey {
    pub title: String,
    pub user_id: Uuid,
    pub survey_data: Value,
}

#[derive(Debug, Deserialize)]
pub struct RequestCreateSurvey {
    pub title: String,
    pub survey_data: Value,
}

#[derive(Debug, Serialize)]
pub struct ResponseSurvey {
    pub id: Uuid,
    pub title: String,
    pub survey_data: Value,
}
