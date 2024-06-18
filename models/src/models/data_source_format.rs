use crate::models;
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

/// DataSourceFormat : Data source format
/// Data source format
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
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataSourceFormat {
    Delta,
    Csv,
    Json,
    Avro,
    Parquet,
    Orc,
    Text,
}
