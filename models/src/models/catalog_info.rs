use crate::models;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogInfo {
    /// Name of catalog.
    pub name: String,
    /// User-provided free-form text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// A map of key-value properties attached to the securable.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
    /// Time at which this catalog was created, in epoch milliseconds.
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    /// Time at which this catalog was last modified, in epoch milliseconds.
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
    /// Unique identifier for the catalog.
    pub id: Uuid,
}
