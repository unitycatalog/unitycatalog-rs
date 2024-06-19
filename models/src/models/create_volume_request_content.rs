use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVolumeRequestContent {
    /// The name of the catalog where the schema and the volume are
    pub catalog_name: String,
    /// The name of the schema where the volume is
    pub schema_name: String,
    /// The name of the volume
    pub name: String,
    pub volume_type: models::VolumeType,
    /// The comment attached to the volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The storage location of the volume
    pub storage_location: String,
}
