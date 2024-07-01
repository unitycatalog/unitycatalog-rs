use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, Router},
    Json,
};

use serde::{Deserialize, Serialize};
use unitycatalog_models::models::{CreateFunction, FunctionInfo, ListFunctionsResponse};
use utoipa::IntoParams;

use crate::state::AppState;

use super::PaginationParams;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_functions).post(post_function))
        .route("/:name", get(get_function).delete(delete_function))
        .with_state(state)
}

#[derive(Debug, Clone, Deserialize, Serialize, IntoParams)]
pub struct ListFunctionParams {
    catalog_name: String,
    schema_name: String,
    #[serde(flatten)]
    pagination: PaginationParams,
}

#[utoipa::path(
    post,
    path = "/",
    operation_id = "listFunctions",
    responses(
        (status = 200, description = "The function list was successfully retrieved.")
    )
)]
async fn get_functions(
    State(state): State<AppState>,
    Query(pagination): Query<ListFunctionParams>,
) -> Json<ListFunctionsResponse> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/",
    operation_id = "createFunction",
    responses(
        (status = 200, description = "The new function was successfully created.", body = FunctionInfo)
    )
)]
async fn post_function(
    State(state): State<AppState>,
    Json(body): Json<CreateFunction>,
) -> Json<FunctionInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{name}",
    operation_id = "getFunction",
    responses(
        (status = 200, description = "The function was successfully retrieved.", body = FunctionInfo)
    )
)]
async fn get_function(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Json<FunctionInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{name}",
    operation_id = "deleteFunction",
    responses(
        (status = 200, description = "The function was successfully deleted.")
    )
)]
async fn delete_function(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    todo!()
}
