use crate::models;
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFunction {
    /// Name of function, relative to parent schema.
    pub name: String,
    /// Name of parent catalog.
    pub catalog_name: String,
    /// Name of parent schema relative to its parent catalog.
    pub schema_name: String,
    pub input_params: models::FunctionParameterInfos,
    pub data_type: models::ColumnTypeName,
    /// Pretty printed function data type.
    pub full_data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_params: Option<models::FunctionParameterInfos>,
    /// Function language. When **EXTERNAL** is used, the language of the routine function should be specified in the __external_language__ field,  and the __return_params__ of the function cannot be used (as **TABLE** return type is not supported), and the __sql_data_access__ field must be **NO_SQL**.
    pub routine_body: RoutineBody,
    /// Function body.
    pub routine_definition: String,
    #[serde(
        skip_serializing_if = "Option::is_none"
    )]
    pub routine_dependencies: Option<models::DependencyList>,
    /// Function parameter style. **S** is the value for SQL.
    pub parameter_style: ParameterStyle,
    /// Whether the function is deterministic.
    pub is_deterministic: bool,
    /// Function SQL data access.
    pub sql_data_access: SqlDataAccess,
    /// Function null call.
    pub is_null_call: bool,
    /// Function security type.
    pub security_type: SecurityType,
    /// Specific name of the function; Reserved for future use.
    pub specific_name: String,
    /// User-provided free-form text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// JSON-serialized key-value pair map, encoded (escaped) as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    /// External language of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_language: Option<String>,
}

/// Function language. When **EXTERNAL** is used, the language of the routine function should be specified in the __external_language__ field,  and the __return_params__ of the function cannot be used (as **TABLE** return type is not supported), and the __sql_data_access__ field must be **NO_SQL**.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Display, IntoStaticStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum RoutineBody {
    Sql,
    External,
}

/// Function parameter style. **S** is the value for SQL.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Display, IntoStaticStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ParameterStyle {
    S,
}

impl Default for ParameterStyle {
    fn default() -> ParameterStyle {
        Self::S
    }
}
/// Function SQL data access.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Display, IntoStaticStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SqlDataAccess {
    ContainsSql,
    ReadsSqlData,
    NoSql,
}

/// Function security type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Display, IntoStaticStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityType {
    Definer,
}
