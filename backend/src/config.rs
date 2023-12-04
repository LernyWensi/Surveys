use std::{env, str::FromStr};

use crate::{Error, Result};

#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Config {
    pub DATABASE_URL: String,
    pub JWT_SECRET: String,
    pub JWT_EXPIRES_IN: String,
    pub JWT_MAXAGE_MINUTES: i64,
}

impl Config {
    pub fn init() -> Result<Config> {
        Ok(Config {
            DATABASE_URL: get_env("DATABASE_URL")?,
            JWT_SECRET: get_env("JWT_SECRET")?,
            JWT_EXPIRES_IN: get_env("JWT_EXPIRES_IN")?,
            JWT_MAXAGE_MINUTES: get_env("JWT_MAXAGE_MINUTES")?,
        })
    }
}

fn get_env<T: FromStr>(name: &'static str) -> Result<T> {
    let value = env::var(name).map_err(|_| Error::EnvMissingValue(name))?;
    value.parse::<T>().map_err(|_| Error::EnvWrongFormat(name))
}
