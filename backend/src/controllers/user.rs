use uuid::Uuid;

use super::base::{self, BaseController};
use crate::{
    models::user::{CreateUser, User},
    AppState, Error, Result,
};

pub struct UserController;

impl BaseController for UserController {
    const TABLE: &'static str = "user";
}

impl UserController {
    pub async fn get_by_id(app_state: &AppState, id: &Uuid) -> Result<User> {
        base::get::<Self, _, _>(app_state, "id", id).await
    }

    pub async fn get_by_name(app_state: &AppState, name: &str) -> Result<User> {
        base::get::<Self, _, _>(app_state, "name", &name).await
    }

    pub async fn exists_by_name(app_state: &AppState, name: &str) -> Result<bool> {
        base::exists::<Self, _>(app_state, "name", &name).await
    }

    pub async fn create(app_state: &AppState, new_user: &CreateUser) -> Result<User> {
        let query = format!(
            "insert into \"{}\" (name, password) values ($1, $2) returning *",
            Self::TABLE
        );

        let user: User = sqlx::query_as(&query)
            .bind(&new_user.name)
            .bind(&new_user.password)
            .fetch_optional(&app_state.db)
            .await?
            .ok_or(Error::DbFailedToCreateEntity(Self::TABLE))?;

        Ok(user)
    }
}
