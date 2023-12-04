use super::base::BaseController;
use crate::{
    models::survey_result::{CreateSurveyResult, SurveyResult},
    AppState, Error, Result,
};

pub struct SurveyResultController;

impl BaseController for SurveyResultController {
    const TABLE: &'static str = "result";
}

impl SurveyResultController {
    pub async fn create(
        app_state: &AppState,
        new_result: &CreateSurveyResult,
    ) -> Result<SurveyResult> {
        let query = format!(
            "insert into \"{}\" (survey_id, result_data) values ($1, $2) returning *",
            Self::TABLE
        );

        let result: SurveyResult = sqlx::query_as(&query)
            .bind(new_result.survey_id)
            .bind(&new_result.result_data)
            .fetch_optional(&app_state.db)
            .await?
            .ok_or(Error::DbFailedToCreateEntity(Self::TABLE))?;

        Ok(result)
    }
}
