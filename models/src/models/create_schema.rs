use crate::models;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSchema {
    /// Name of schema, relative to parent catalog.
    pub name: String,
    /// Name of parent catalog.
    pub catalog_name: String,
    /// User-provided free-form text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// A map of key-value properties attached to the securable.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
}
