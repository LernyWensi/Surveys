use crate::AppState;
use crate::Error;
use crate::Result;
use sqlx::Encode;
use sqlx::Postgres;
use sqlx::Type;
use sqlx::{postgres::PgRow, FromRow};

pub trait BaseController {
    const TABLE: &'static str;
}

pub async fn get<C, T, D>(app_state: &AppState, column: &str, target: &T) -> Result<D>
where
    C: BaseController,
    T: for<'q> Encode<'q, Postgres> + Type<Postgres> + Send + Sync + ToString,
    D: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let query = format!("select * from \"{}\" where {column} = $1", C::TABLE);

    let entity = sqlx::query_as(&query)
        .bind(target)
        .fetch_optional(&app_state.db)
        .await?
        .ok_or(Error::DbEntityNotFound {
            entity: C::TABLE,
            column: column.to_owned(),
            target: target.to_string(),
        })?;

    Ok(entity)
}

pub async fn get_many<C, T, D>(app_state: &AppState, column: &str, target: &T) -> Result<Vec<D>>
where
    C: BaseController,
    T: for<'q> Encode<'q, Postgres> + Type<Postgres> + Send + Sync + ToString,
    D: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let query = format!("select * from \"{}\" where {column} = $1", C::TABLE);

    let entities = sqlx::query_as(&query)
        .bind(target)
        .fetch_all(&app_state.db)
        .await?;

    Ok(entities)
}

pub async fn exists<C, T>(app_state: &AppState, column: &str, target: &T) -> Result<bool>
where
    C: BaseController,
    T: for<'q> Encode<'q, Postgres> + Type<Postgres> + Send + Sync,
{
    let query = format!("select 1 from \"{}\" where {column} = $1", C::TABLE);

    let exists = sqlx::query(&query)
        .bind(target)
        .fetch_optional(&app_state.db)
        .await?;

    Ok(exists.is_some())
}
