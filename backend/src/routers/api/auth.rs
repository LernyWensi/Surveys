use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{extract::State, http::StatusCode, middleware, routing::post, Json, Router};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use jsonwebtoken::{EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;
use std::sync::Arc;
use time::OffsetDateTime;

use crate::{
    controllers::user::UserController,
    mv::auth::{auth_require, AuthenticatedUser, AUTH_TOKEN},
    models::{
        user::{CreateUser, RequestLoginUser, RequestSignupUser, ResponseUser, TokenClaims},
        DefaultResponse,
    },
    AppState, Error, Result,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/signup", post(signup_handler))
        .route("/login", post(login_handler))
        .route(
            "/logout",
            post(logout_handler).route_layer(middleware::from_fn(auth_require)),
        )
}

async fn signup_handler(
    State(app_state): State<Arc<AppState>>,
    Json(req_body): Json<RequestSignupUser>,
) -> Result<(StatusCode, Json<DefaultResponse>)> {
    tracing::debug!("{:-<15} |> signup_handler |> {}", "HANDLER ", file!());

    let user_exists = UserController::exists_by_name(&app_state, &req_body.name).await?;

    if user_exists {
        return Err(Error::RegistrationNotUniqueUsername(
            req_body.name.to_owned(),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(req_body.password.as_bytes(), &salt)?
        .to_string();

    let user: ResponseUser = UserController::create(
        &app_state,
        &CreateUser {
            name: req_body.name,
            password: hashed_password,
        },
    )
    .await?
    .filter_for_response();

    let res_body = DefaultResponse {
        success: true,
        message: "User registered in successfully".to_owned(),
        data: Some(json!({
            "user": user,
        })),
        error: None,
    };

    Ok((StatusCode::CREATED, Json(res_body)))
}

async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
    Json(req_body): Json<RequestLoginUser>,
) -> Result<(CookieJar, Json<DefaultResponse>)> {
    tracing::debug!("{:-<15} |> login_handler |> {}", "HANDLER ", file!());

    let user = UserController::get_by_name(&app_state, &req_body.name).await?;

    PasswordHash::new(&user.password).and_then(|parsed_hash| {
        Argon2::default().verify_password(req_body.password.as_bytes(), &parsed_hash)
    })?;

    let now = OffsetDateTime::now_utc();
    let claims = TokenClaims {
        sub: user.id.to_string(),
        exp: (now + time::Duration::minutes(app_state.config.JWT_MAXAGE_MINUTES)).unix_timestamp()
            as usize,
        iat: now.unix_timestamp() as usize,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_state.config.JWT_SECRET.as_ref()),
    )?;

    let cookie = Cookie::build((AUTH_TOKEN, token.clone()))
        .path("/")
        .max_age(time::Duration::minutes(app_state.config.JWT_MAXAGE_MINUTES))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let res_body = DefaultResponse {
        success: true,
        message: "User logged in successfully".to_owned(),
        data: Some(json!({
            "user": user.filter_for_response(),
            "token": token
        })),
        error: None,
    };

    Ok((cookie_jar.add(cookie), Json(res_body)))
}

async fn logout_handler(
    State(app_state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
    user: AuthenticatedUser,
) -> Result<(CookieJar, Json<DefaultResponse>)> {
    tracing::debug!("{:-<15} |> logout_handler |> {}", "HANDLER ", file!());

    let cookie = Cookie::build(AUTH_TOKEN)
        .path("/")
        .max_age(time::Duration::minutes(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let res_body = DefaultResponse {
        success: true,
        message: "User logged out successfully".to_owned(),
        data: Some(json!(UserController::get_by_id(&app_state, &user.id)
            .await?
            .filter_for_response())),
        error: None,
    };

    Ok((cookie_jar.remove(cookie), Json(res_body)))
}
