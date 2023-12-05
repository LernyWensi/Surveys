use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    http::{header, request::Parts, HeaderValue},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use jsonwebtoken::{DecodingKey, Validation};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{models::user::TokenClaims, AppState, Error, Result};

pub const AUTH_TOKEN: &str = "auth-token";

#[derive(Debug, Serialize, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
}

pub async fn auth_require(
    authenticated_user: Result<AuthenticatedUser>,
    req: Request,
    next: Next,
) -> Result<Response> {
    tracing::debug!("{:-<15} |> auth_require |> {}", "MIDDLEWARE ", file!());

    authenticated_user?;

    Ok(next.run(req).await)
}

pub async fn auth(
    app_state: State<Arc<AppState>>,
    mut cookie_jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<(CookieJar, Response)> {
    tracing::debug!("{:-<15} |> auth |> {}", "MIDDLEWARE ", file!());

    let auth_header = req.headers().get(header::AUTHORIZATION);
    let auth_result = _auth(app_state, &cookie_jar, auth_header).await;

    if auth_result.is_err() && !matches!(auth_result, Err(Error::AuthMissingToken)) {
        cookie_jar = cookie_jar.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(auth_result);

    Ok((cookie_jar, next.run(req).await))
}

async fn _auth(
    State(app_state): State<Arc<AppState>>,
    cookie_jar: &CookieJar,
    auth_header: Option<&HeaderValue>,
) -> Result<AuthenticatedUser> {
    let token = cookie_jar
        .get(AUTH_TOKEN)
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            auth_header
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| auth_value.strip_prefix("Bearer ").map(|t| t.to_owned()))
        });

    let token = token.ok_or(Error::AuthMissingToken)?;

    let claims = jsonwebtoken::decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(app_state.config.JWT_SECRET.as_ref()),
        &Validation::default(),
    )?
    .claims;

    let user_id = Uuid::parse_str(&claims.sub)?;

    Ok(AuthenticatedUser { id: user_id })
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AuthenticatedUser {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        tracing::debug!("{:-<15} |> AuthenticatedUser |> {}", "EXTRACTOR ", file!());

        parts
            .extensions
            .get::<Result<AuthenticatedUser>>()
            .unwrap()
            .to_owned()
    }
}
