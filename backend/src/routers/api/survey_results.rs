use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde_json::json;

use crate::{
    controllers::survey_result::SurveyResultController,
    models::{
        survey_result::{CreateSurveyResult, RequestCreateSurveyResult},
        DefaultResponse,
    },
    AppState, Result,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/results", post(create_result_handler))
}

async fn create_result_handler(
    State(app_state): State<Arc<AppState>>,
    Json(req_body): Json<RequestCreateSurveyResult>,
) -> Result<(StatusCode, Json<DefaultResponse>)> {
    tracing::debug!(
        "{:-<15} |> create_result_handler |> {}",
        "HANDLER ",
        file!()
    );

    let new_result = CreateSurveyResult {
        survey_id: req_body.survey_id,
        result_data: req_body.result_data,
    };

    let result = SurveyResultController::create(&app_state, &new_result).await?;

    let res_body = DefaultResponse {
        success: true,
        message: "Query executed successfully, returning newly created result".to_owned(),
        data: Some(json!({
            "survey_id": req_body.survey_id,
            "result": json!(result.filter_for_response())
        })),
        error: None,
    };

    Ok((StatusCode::CREATED, Json(res_body)))
}
