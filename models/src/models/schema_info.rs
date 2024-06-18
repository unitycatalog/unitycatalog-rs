use crate::models;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SchemaInfo {
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
    /// Full name of schema, in form of __catalog_name__.__schema_name__.
    pub full_name: String,
    /// Time at which this schema was created, in epoch milliseconds.
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    /// Time at which this schema was last modified, in epoch milliseconds.
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
    /// Unique identifier for the schema.
    pub schema_id: Uuid,
}
