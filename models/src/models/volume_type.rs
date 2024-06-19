use crate::models;
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};

/// VolumeType : The type of the volume
/// The type of the volume
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
pub enum VolumeType {
    Managed,
    External,
}
