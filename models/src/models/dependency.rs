use crate::models;
use serde::{Deserialize, Serialize};

/// Dependency : A dependency of a SQL object. Either the __table__ field or the __function__ field must be defined.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dependency {
    Table(models::TableDependency),
    Function(models::FunctionDependency),
}
