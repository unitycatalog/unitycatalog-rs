use crate::models;
use serde::{Deserialize, Serialize};

/// FunctionDependency : A function that is dependent on a SQL object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionDependency {
    /// Full name of the dependent function, in the form of __catalog_name__.__schema_name__.__function_name__.
    pub function_full_name: String,
}