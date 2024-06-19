use crate::models;
use serde::{Deserialize, Serialize};

/// TableDependency : A table that is dependent on a SQL object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableDependency {
    /// Full name of the dependent table, in the form of __catalog_name__.__schema_name__.__table_name__.
    pub table_full_name: String,
}
