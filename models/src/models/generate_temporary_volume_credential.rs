use crate::models;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateTemporaryVolumeCredential {
    /// Volume id for which temporary credentials are generated.  Can be obtained from volumes/{full_name} (get volume info) API.
    pub volume_id: Uuid,
    pub operation: models::VolumeOperation,
}
