use axum::Router;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

use crate::state::AppState;

mod functions;
mod catalogs;
mod schemas;
mod tables;
mod volumes;

#[derive(Debug, Clone, Deserialize, Serialize, IntoParams)]
pub struct PaginationParams {
    pub page_token: String,
    pub max_results: u32,
}

pub fn all_routes(state: AppState) -> Router {
    Router::new()
        .nest("/functions", functions::router(state.clone()))
        .nest("/catalogs", catalogs::router(state.clone()))
        .nest("/schemas", schemas::router(state.clone()))
        .nest("/tables", tables::router(state.clone()))
        .nest("/volumes", volumes::router(state.clone()))
}