use crate::models;
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

/// FunctionParameterMode : The mode of the function parameter.
/// The mode of the function parameter.
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
pub enum FunctionParameterMode {
    In,
}
