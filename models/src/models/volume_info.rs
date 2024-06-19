use crate::models;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeInfo {
    /// The name of the catalog where the schema and the volume are
    pub catalog_name: String,
    /// The name of the schema where the volume is
    pub schema_name: String,
    /// The name of the volume
    pub name: String,
    /// The comment attached to the volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Time at which this volume was created, in epoch milliseconds.
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    /// Time at which this volume was last modified, in epoch milliseconds.
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
    /// Unique identifier for the volume
    pub volume_id: Uuid,
    pub volume_type: models::VolumeType,
    /// The storage location of the volume
    pub storage_location: String,
    /// Full name of volume, in form of __catalog_name__.__schema_name__.__volume_name__.
    pub full_name: String,
}
