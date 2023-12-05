use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    controllers::survey::SurveyController,
    models::{
        survey::{CreateSurvey, RequestCreateSurvey, ResponseSurvey},
        DefaultResponse,
    },
    mv::auth::{auth_require, AuthenticatedUser},
    AppState, Result,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/surveys", get(get_user_surveys_handler))
        .route("/surveys/:id", get(get_user_surveys_by_id))
        .route(
            "/surveys",
            post(create_survey).route_layer(middleware::from_fn(auth_require)),
        )
}

async fn get_user_surveys_by_id(
    Path(survey_id): Path<Uuid>,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<DefaultResponse>> {
    tracing::debug!(
        "{:-<15} |> get_user_surveys_by_id |> {}",
        "HANDLER ",
        file!()
    );

    let survey = SurveyController::get_by_id(&app_state, &survey_id).await?;

    let res_body = DefaultResponse {
        success: true,
        message: format!("Query executed successfully, returning survey with id {survey_id}"),
        data: Some(json!({
            "survey": json!(survey.filter_for_response())
        })),
        error: None,
    };

    Ok(Json(res_body))
}

async fn get_user_surveys_handler(
    user: AuthenticatedUser,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<DefaultResponse>> {
    tracing::debug!(
        "{:-<15} |> get_user_surveys_handler |> {}",
        "HANDLER ",
        file!()
    );

    let surveys = SurveyController::get_all_by_user_id(&app_state, &user.id).await?;
    let s: Vec<ResponseSurvey> = surveys.iter().map(|s| s.filter_for_response()).collect();

    let res_body = DefaultResponse {
        success: true,
        message: "Query executed successfully, returning surveys of user".to_owned(),
        data: Some(json!({
            "user": user.id,
            "surveys": json!(s)
        })),
        error: None,
    };

    Ok(Json(res_body))
}

async fn create_survey(
    user: AuthenticatedUser,
    State(app_state): State<Arc<AppState>>,
    Json(new_survey): Json<RequestCreateSurvey>,
) -> Result<(StatusCode, Json<DefaultResponse>)> {
    tracing::debug!("{:-<15} |> create_survey |> {}", "HANDLER ", file!());

    let new_survey = CreateSurvey {
        user_id: user.id,
        title: new_survey.title,
        survey_data: new_survey.survey_data,
    };

    let survey = SurveyController::create(&app_state, &new_survey).await?;

    let res_body = DefaultResponse {
        success: true,
        message: "Query executed successfully, returning newly created survey".to_owned(),
        data: Some(json!({
            "user": user.id,
            "surveys": json!(survey.filter_for_response())
        })),
        error: None,
    };

    Ok((StatusCode::CREATED, Json(res_body)))
}
