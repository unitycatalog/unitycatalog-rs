use crate::models;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTable {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    /// User-provided free-form text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// A map of key-value properties attached to the securable.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
}
