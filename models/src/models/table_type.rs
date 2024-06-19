use crate::models;
use serde::{Deserialize, Serialize};

#[allow(clippy::empty_docs)]
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TableType {
    #[serde(rename = "MANAGED")]
    Managed,
    #[serde(rename = "EXTERNAL")]
    External,
}

impl std::fmt::Display for TableType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Managed => write!(f, "MANAGED"),
            Self::External => write!(f, "EXTERNAL"),
        }
    }
}

impl Default for TableType {
    fn default() -> TableType {
        Self::Managed
    }
}
