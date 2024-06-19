use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnInfo {
    /// Name of Column.
    pub name: String,
    /// Full data type specification as SQL/catalogString text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_text: Option<String>,
    /// Full data type specification, JSON-serialized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_json: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<models::ColumnTypeName>,
    /// Digits of precision; required for DecimalTypes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_precision: Option<i32>,
    /// Digits to right of decimal; Required for DecimalTypes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_scale: Option<i32>,
    /// Format of IntervalType.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_interval_type: Option<String>,
    /// Ordinal position of column (starting at position 0).
    pub position: u32,
    /// User-provided free-form text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Whether field may be Null.
    pub nullable: bool,
    /// Partition index for column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_index: Option<i32>,
}
