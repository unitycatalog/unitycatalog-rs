use crate::models;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCatalog {
    /// Name of catalog.
    pub name: String,
    /// User-provided free-form text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// A map of key-value properties attached to the securable.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
}

impl CreateCatalog {
    pub fn build(self) -> models::CatalogInfo {
        let current_time = chrono::Utc::now();
        models::CatalogInfo {
            name: self.name,
            comment: self.comment,
            created_at: current_time,
            updated_at: current_time,
            id: uuid::Uuid::new_v4(),
            properties: self.properties,
        }
    }
}

impl From<models::CreateCatalog> for models::CatalogInfo {
    fn from(catalog: models::CreateCatalog) -> Self {
        catalog.build()
    }
}