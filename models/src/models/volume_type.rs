use crate::models;
use serde::{Deserialize, Serialize};

/// VolumeType : The type of the volume
/// The type of the volume
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VolumeType {
    #[serde(rename = "MANAGED")]
    Managed,
    #[serde(rename = "EXTERNAL")]
    External,
}

impl std::fmt::Display for VolumeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Managed => write!(f, "MANAGED"),
            Self::External => write!(f, "EXTERNAL"),
        }
    }
}

impl Default for VolumeType {
    fn default() -> VolumeType {
        Self::Managed
    }
}
