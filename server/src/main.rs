#![allow(unused)]

use crate::routes::all_routes;
use crate::state::AppState;
use anyhow::Result;
use axum::Router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "unitycatalog=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let state = AppState {};

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;
    let app = Router::new().nest("/api/2.1/unity-catalog/", all_routes(state));
    axum::serve(listener, app).await?;
    Ok(())
}
