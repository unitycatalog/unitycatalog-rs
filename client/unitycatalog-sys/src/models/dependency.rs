/*
 * Unity Catalog API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Dependency : A dependency of a SQL object. Either the __table__ field or the __function__ field must be defined.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dependency {
    #[serde(rename = "table", skip_serializing_if = "Option::is_none")]
    pub table: Option<Box<models::TableDependency>>,
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<Box<models::FunctionDependency>>,
}

impl Dependency {
    /// A dependency of a SQL object. Either the __table__ field or the __function__ field must be defined.
    pub fn new() -> Dependency {
        Dependency {
            table: None,
            function: None,
        }
    }
}

