use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, Router},
    Json,
};

use serde::{Deserialize, Serialize};
use unitycatalog_models::models::{CreateVolumeRequestContent, ListVolumesResponseContent, VolumeInfo};
use utoipa::IntoParams;

use crate::state::AppState;

use super::PaginationParams;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_volumes).post(post_volume))
        .route("/:name", get(get_volume).delete(delete_volume))
        .with_state(state)
}

#[derive(Debug, Clone, Deserialize, Serialize, IntoParams)]
pub struct ListVolumeParams {
    catalog_name: String,
    schema_name: String,
    #[serde(flatten)]
    pagination: PaginationParams,
}

#[utoipa::path(
    post,
    path = "/",
    operation_id = "listVolumes",
    responses(
        (status = 200, description = "The volumes list was successfully retrieved.", body = ListVolumesResponse)
    )
)]
async fn get_volumes(
    State(state): State<AppState>,
    Query(pagination): Query<ListVolumeParams>,
) -> Json<ListVolumesResponseContent> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/",
    operation_id = "createVolume",
    responses(
        (status = 200, description = "The new volume was successfully created.", body = VolumeInfo)
    )
)]
async fn post_volume(
    State(state): State<AppState>,
    Json(body): Json<CreateVolumeRequestContent>,
) -> Json<VolumeInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{full_name}",
    operation_id = "getVolume",
    responses(
        (status = 200, description = "The volume was successfully retrieved.", body = VolumeInfo)
    )
)]
async fn get_volume(State(state): State<AppState>, Path(name): Path<String>) -> Json<VolumeInfo> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/{full_name}",
    operation_id = "deleteVolume",
    responses(
        (status = 200, description = "The volume was successfully deleted.")
    )
)]
async fn delete_volume(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    todo!()
}
