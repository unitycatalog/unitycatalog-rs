use crate::models;
use serde::{Deserialize, Serialize};

/// DependencyList : A list of dependencies.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DependencyList {
    /// Array of dependencies.
    pub dependencies: Vec<models::Dependency>,
}
