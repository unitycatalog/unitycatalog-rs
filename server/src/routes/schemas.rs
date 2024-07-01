use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, Router},
    Json,
};

use serde::{Deserialize, Serialize};
use unitycatalog_models::models::{CreateSchema, ListSchemasResponse, SchemaInfo, UpdateCatalog};

use crate::state::AppState;

use super::PaginationParams;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_schemas).post(post_schema))
        .route(
            "/:name",
            get(get_schema).patch(patch_schema).delete(delete_schema),
        )
        .with_state(state)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListSchemaParams {
    catalog_name: String,
    #[serde(flatten)]
    pagination: PaginationParams,
}

#[utoipa::path(
    post,
    path = "/",
    operation_id = "listSchemas",
    responses(
        (status = 200, description = "The schemas list was successfully retrieved.")
    )
)]
async fn get_schemas(
    State(state): State<AppState>,
    Query(pagination): Query<ListSchemaParams>,
) -> Json<ListSchemasResponse> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/",
    operation_id = "createSchema",
    responses(
        (status = 200, description = "The new schema was successfully created.", body = CatalogInfo)
    )
)]
async fn post_schema(
    State(state): State<AppState>,
    Json(body): Json<CreateSchema>,
) -> Json<SchemaInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{full_name}",
    operation_id = "getSchema",
    responses(
        (status = 200, description = "The schema was successfully retrieved.", body = SchemaInfo)
    )
)]
async fn get_schema(State(state): State<AppState>, Path(name): Path<String>) -> Json<SchemaInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{full_name}",
    operation_id = "updateSchema",
    responses(
        (status = 200, description = "The schema was successfully updated.", body = SchemaInfo)
    )
)]
async fn patch_schema(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(body): Json<UpdateCatalog>,
) -> Json<SchemaInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{full_name}",
    operation_id = "deleteSchema",
    responses(
        (status = 200, description = "The schema was successfully deleted.")
    )
)]
async fn delete_schema(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    todo!()
}
