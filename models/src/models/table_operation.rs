use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TableOperation {
    #[serde(rename = "UNKNOWN_TABLE_OPERATION")]
    UnknownTableOperation,
    #[serde(rename = "READ")]
    Read,
    #[serde(rename = "READ_WRITE")]
    ReadWrite,
}

impl std::fmt::Display for TableOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnknownTableOperation => write!(f, "UNKNOWN_TABLE_OPERATION"),
            Self::Read => write!(f, "READ"),
            Self::ReadWrite => write!(f, "READ_WRITE"),
        }
    }
}

impl Default for TableOperation {
    fn default() -> TableOperation {
        Self::UnknownTableOperation
    }
}
