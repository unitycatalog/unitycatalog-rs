use crate::models;
use serde::{Deserialize, Serialize};

/// FunctionParameterMode : The mode of the function parameter.
/// The mode of the function parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FunctionParameterMode {
    #[serde(rename = "IN")]
    In,
}

impl std::fmt::Display for FunctionParameterMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::In => write!(f, "IN"),
        }
    }
}

impl Default for FunctionParameterMode {
    fn default() -> FunctionParameterMode {
        Self::In
    }
}
