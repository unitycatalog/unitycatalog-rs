use crate::models;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateTemporaryTableCredential {
    /// Table id for which temporary credentials are generated.  Can be obtained from tables/{full_name} (get table info) API.
    pub table_id: Uuid,
    pub operation: models::TableOperation,
}
