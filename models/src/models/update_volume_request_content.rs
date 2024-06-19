use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateVolumeRequestContent {
    /// The comment attached to the volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// New name for the volume.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
}
