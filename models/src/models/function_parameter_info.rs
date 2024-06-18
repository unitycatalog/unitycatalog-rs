use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionParameterInfo {
    /// Name of parameter.
    pub name: String,
    /// Full data type spec, SQL/catalogString text.
    pub type_text: String,
    /// Full data type spec, JSON-serialized.
    pub type_json: String,
    pub type_name: models::ColumnTypeName,
    /// Digits of precision; required on Create for DecimalTypes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_precision: Option<i32>,
    /// Digits to right of decimal; Required on Create for DecimalTypes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_scale: Option<i32>,
    /// Format of IntervalType.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_interval_type: Option<String>,
    /// Ordinal position of column (starting at position 0).
    pub position: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_mode: Option<models::FunctionParameterMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<models::FunctionParameterType>,
    /// Default value of the parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_default: Option<String>,
    /// User-provided free-form text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
