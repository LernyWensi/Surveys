use uuid::Uuid;

use super::base::{self, BaseController};
use crate::{
    models::survey::{CreateSurvey, Survey},
    AppState, Error, Result,
};

pub struct SurveyController;

impl BaseController for SurveyController {
    const TABLE: &'static str = "survey";
}

impl SurveyController {
    pub async fn get_by_id(app_state: &AppState, id: &Uuid) -> Result<Survey> {
        base::get::<Self, _, _>(app_state, "id", id).await
    }

    pub async fn get_all_by_user_id(app_state: &AppState, user_id: &Uuid) -> Result<Vec<Survey>> {
        base::get_many::<Self, _, _>(app_state, "user_id", user_id).await
    }

    pub async fn create(app_state: &AppState, new_survey: &CreateSurvey) -> Result<Survey> {
        let query = format!(
            "insert into \"{}\" (title, user_id, survey_data) values ($1, $2, $3) returning *",
            Self::TABLE
        );

        let survey: Survey = sqlx::query_as(&query)
            .bind(&new_survey.title)
            .bind(new_survey.user_id)
            .bind(&new_survey.survey_data)
            .fetch_optional(&app_state.db)
            .await?
            .ok_or(Error::DbFailedToCreateEntity(Self::TABLE))?;

        Ok(survey)
    }
}
