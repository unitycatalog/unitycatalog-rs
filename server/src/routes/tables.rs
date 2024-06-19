use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, Router},
    Json,
};

use serde::{Deserialize, Serialize};
use unitycatalog_models::models::{CreateTable, ListTablesResponse, TableInfo};
use utoipa::IntoParams;

use crate::state::AppState;

use super::PaginationParams;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_tables).post(post_table))
        .route("/:name", get(get_table).delete(delete_table))
        .with_state(state)
}

#[derive(Debug, Clone, Deserialize, Serialize, IntoParams)]
pub struct ListTableParams {
    catalog_name: String,
    schema_name: String,
    #[serde(flatten)]
    pagination: PaginationParams,
}

#[utoipa::path(
    post,
    path = "/",
    operation_id = "listTables",
    responses(
        (status = 200, description = "The tables list was successfully retrieved.", body = ListTablesResponse)
    )
)]
async fn get_tables(
    State(state): State<AppState>,
    Query(pagination): Query<ListTableParams>,
) -> Json<ListTablesResponse> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/",
    operation_id = "createTable",
    responses(
        (status = 200, description = "The new table was successfully created.", body = TableInfo)
    )
)]
async fn post_table(
    State(state): State<AppState>,
    Json(body): Json<CreateTable>,
) -> Json<TableInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{full_name}",
    operation_id = "getTable",
    responses(
        (status = 200, description = "The table was successfully retrieved.", body = TableInfo)
    )
)]
async fn get_table(State(state): State<AppState>, Path(name): Path<String>) -> Json<TableInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{full_name}",
    operation_id = "deleteTable",
    responses(
        (status = 200, description = "The table was successfully deleted.")
    )
)]
async fn delete_table(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    todo!()
}
