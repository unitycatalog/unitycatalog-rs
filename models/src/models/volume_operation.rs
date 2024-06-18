use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VolumeOperation {
    #[serde(rename = "UNKNOWN_VOLUME_OPERATION")]
    UnknownVolumeOperation,
    #[serde(rename = "READ_VOLUME")]
    ReadVolume,
    #[serde(rename = "WRITE_VOLUME")]
    WriteVolume,
}

impl std::fmt::Display for VolumeOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownVolumeOperation => write!(f, "UNKNOWN_VOLUME_OPERATION"),
            Self::ReadVolume => write!(f, "READ_VOLUME"),
            Self::WriteVolume => write!(f, "WRITE_VOLUME"),
        }
    }
}

impl Default for VolumeOperation {
    fn default() -> VolumeOperation {
        Self::UnknownVolumeOperation
    }
}
