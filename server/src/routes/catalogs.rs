use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, Router},
    Json,
};

use unitycatalog_models::models::{
    CatalogInfo, CreateCatalog, ListCatalogsResponse, UpdateCatalog,
};

use crate::state::AppState;

use super::PaginationParams;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_catalogs).post(post_catalog))
        .route(
            "/:name",
            get(get_catalog).patch(get_catalog).delete(delete_catalog),
        )
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "/",
    operation_id = "listCatalogs",
    responses(
        (status = 200, description = "The catalog list was successfully retrieved.")
    )
)]
async fn get_catalogs(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> Json<ListCatalogsResponse> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/",
    operation_id = "createCatalog",
    responses(
        (status = 200, description = "The new catalog was successfully created.", body = CatalogInfo)
    )
)]
async fn post_catalog(
    State(state): State<AppState>,
    Json(body): Json<CreateCatalog>,
) -> Json<CatalogInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{name}",
    operation_id = "getCatalog",
    responses(
        (status = 200, description = "The catalog was successfully retrieved.", body = CatalogInfo)
    )
)]
async fn get_catalog(State(state): State<AppState>, Path(name): Path<String>) -> Json<CatalogInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{name}",
    operation_id = "updateCatalog",
    responses(
        (status = 200, description = "The catalog was successfully updated.", body = CatalogInfo)
    )
)]
async fn patch_catalog(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(body): Json<UpdateCatalog>,
) -> Json<CatalogInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{name}",
    operation_id = "deleteCatalog",
    responses(
        (status = 200, description = "The catalog was successfully deleted.")
    )
)]
async fn delete_catalog(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    todo!()
}
