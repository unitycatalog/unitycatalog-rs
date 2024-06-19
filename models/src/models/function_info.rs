use crate::models;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// Name of function, relative to parent schema.
    pub name: String,
    /// Name of parent catalog.
    pub catalog_name: String,
    /// Name of parent schema relative to its parent catalog.
    pub schema_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_params: Option<Box<models::FunctionParameterInfos>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<models::ColumnTypeName>,
    /// Pretty printed function data type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_params: Option<Box<models::FunctionParameterInfos>>,
    /// Function language. When **EXTERNAL** is used, the language of the routine function should be specified in the __external_language__ field,  and the __return_params__ of the function cannot be used (as **TABLE** return type is not supported), and the __sql_data_access__ field must be **NO_SQL**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routine_body: Option<RoutineBody>,
    /// Function body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routine_definition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routine_dependencies: Option<models::DependencyList>,
    /// Function parameter style. **S** is the value for SQL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_style: Option<ParameterStyle>,
    /// Whether the function is deterministic.
    pub is_deterministic: bool,
    /// Function SQL data access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_data_access: Option<SqlDataAccess>,
    /// Function null call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_null_call: Option<bool>,
    /// Function security type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<SecurityType>,
    /// Specific name of the function; Reserved for future use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specific_name: Option<String>,
    /// User-provided free-form text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// JSON-serialized key-value pair map, encoded (escaped) as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    /// Full name of function, in form of __catalog_name__.__schema_name__.__function__name__
    pub full_name: String,
    /// Time at which this function was created, in epoch milliseconds.
    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    /// Time at which this function was last updated, in epoch milliseconds.
    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
    /// Id of Function, relative to parent schema.
    pub function_id: String,
    /// External language of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_language: Option<String>,
}

/// Function language. When **EXTERNAL** is used, the language of the routine function should be specified in the __external_language__ field,  and the __return_params__ of the function cannot be used (as **TABLE** return type is not supported), and the __sql_data_access__ field must be **NO_SQL**.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    IntoStaticStr,
    Display,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoutineBody {
    Sql,
    External,
}

/// Function parameter style. **S** is the value for SQL.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    IntoStaticStr,
    Display,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ParameterStyle {
    S,
}

impl Default for ParameterStyle {
    fn default() -> ParameterStyle {
        Self::S
    }
}
/// Function SQL data access.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    IntoStaticStr,
    Display,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SqlDataAccess {
    ContainsSql,
    ReadsSqlData,
    NoSql,
}

/// Function security type.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    IntoStaticStr,
    Display,
)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityType {
    Definer,
}
