use crate::models;
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

/// ColumnTypeName : Name of type (INT, STRUCT, MAP, etc.).
/// Name of type (INT, STRUCT, MAP, etc.).
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
    Display,
    IntoStaticStr,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ColumnTypeName {
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Date,
    Timestamp,
    TimestampNtz,
    String,
    Binary,
    Decimal,
    Interval,
    Array,
    Struct,
    Map,
    Char,
    Null,
    UserDefinedType,
    TableType,
}
