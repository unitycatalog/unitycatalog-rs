use std::collections::HashMap;

use crate::models;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableInfo {
    /// Name of table, relative to parent schema.
    pub name: String,
    /// Name of parent catalog.
    pub catalog_name: String,
    /// Name of parent schema relative to its parent catalog.
    pub schema_name: String,
    pub table_type: models::TableType,
    pub data_source_format: models::DataSourceFormat,
    /// The array of __ColumnInfo__ definitions of the table's columns.
    pub columns: Vec<models::ColumnInfo>,
    /// Storage root URL for table (for **MANAGED**, **EXTERNAL** tables)
    pub storage_location: String,
    /// User-provided free-form text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// A map of key-value properties attached to the securable.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
    /// Time at which this table was created, in epoch milliseconds.
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    /// Time at which this table was last modified, in epoch milliseconds.
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
    /// Unique identifier for the table.
    pub table_id: Uuid,
}
