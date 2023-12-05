mod config;
mod controllers;
mod error;
mod logger;
mod models;
mod mv;
mod routers;

pub use config::Config;
pub use error::{Error, Result};
pub use mv::auth::auth;

use axum::{middleware, Router};
use mv::response_map::response_map;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};
use tokio::net::TcpListener;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub config: Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();
    logger::init();

    let config = Config::init()?;
    let db = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(500))
        .max_connections(1)
        .connect(&config.DATABASE_URL)
        .await
        .map_err(|err| Error::DbFailedToCreatePool(err.to_string()))?;

    let app_state = Arc::new(AppState { db, config });

    let app = Router::new()
        .merge(routers::create())
        .layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .with_state(app_state.clone())
        .fallback_service(routers::statics::create())
        .layer(middleware::map_response(response_map));

    let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3000);
    let tcp_listener = TcpListener::bind(address).await.unwrap();

    // tracing::info!("🚀 SERVER IS RUNNING");
    tracing::info!("{:-<15} |> http://{address} |> {}\n", "LISTENING ", file!());

    axum::serve(tcp_listener, app).await.unwrap();

    Ok(())
}
