use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    EnvWrongFormat(&'static str),
    EnvMissingValue(&'static str),

    AuthUnauthorized,
    AuthMissingToken,
    AuthInvalidTokenSub,

    DbFailedToCreatePool(String),
    DbFailedSqlQuery(String),
    DbFailedToCreateEntity(&'static str),
    DbEntityNotFound {
        entity: &'static str,
        column: String,
        target: String,
    },

    RegistrationNotUniqueUsername(String),

    Argon2(#[serde_as(as = "DisplayFromStr")] argon2::password_hash::Error),
    JsonWebToken(#[serde_as(as = "DisplayFromStr")] jsonwebtoken::errors::Error),
    Uuid(#[serde_as(as = "DisplayFromStr")] uuid::Error),
}

impl From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
        Self::Uuid(value)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::JsonWebToken(value)
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::Argon2(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::DbFailedSqlQuery(value.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::debug!("{:-<15} |> {self:?} |> {}", "INTO_RESPONSE", file!());

        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
    REGISTRATION_FAIL,
    LOGIN_FAIL,
    NO_AUTH,
    ENTITY_NOT_FOUND {
        entity: &'static str,
        column: String,
        target: String,
    },
    SERVICE_ERROR,
}

impl Error {
    pub fn client_error_and_status(&self) -> (StatusCode, ClientError) {
        match self {
            Error::Uuid(_)
            | Error::EnvMissingValue(_)
            | Error::EnvWrongFormat(_)
            | Error::DbFailedToCreateEntity(_)
            | Error::DbFailedSqlQuery(_)
            | Error::DbFailedToCreatePool(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),

            Error::RegistrationNotUniqueUsername(_) => {
                (StatusCode::BAD_REQUEST, ClientError::REGISTRATION_FAIL)
            }

            Error::AuthUnauthorized | Error::AuthMissingToken | Error::AuthInvalidTokenSub => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }

            Error::DbEntityNotFound {
                entity,
                column,
                target,
            } => match *entity {
                "user" => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
                _ => (
                    StatusCode::BAD_REQUEST,
                    ClientError::ENTITY_NOT_FOUND {
                        entity,
                        column: column.to_owned(),
                        target: target.to_owned(),
                    },
                ),
            },

            Error::Argon2(err) => match err {
                argon2::password_hash::Error::Password => {
                    (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ClientError::SERVICE_ERROR,
                ),
            },

            Error::JsonWebToken(err) => match err.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken
                | jsonwebtoken::errors::ErrorKind::InvalidSignature
                | jsonwebtoken::errors::ErrorKind::ExpiredSignature
                | jsonwebtoken::errors::ErrorKind::InvalidIssuer
                | jsonwebtoken::errors::ErrorKind::InvalidAudience
                | jsonwebtoken::errors::ErrorKind::InvalidSubject => {
                    (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ClientError::SERVICE_ERROR,
                ),
            },
        }
    }
}
