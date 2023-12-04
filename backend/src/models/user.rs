use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

impl User {
    pub fn filter_for_response(&self) -> ResponseUser {
        ResponseUser {
            id: self.id,
            name: self.name.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct CreateUser {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestSignupUser {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestLoginUser {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseUser {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
