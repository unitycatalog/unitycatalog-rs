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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTable {
    /// Name of table, relative to parent schema.
    #[serde(rename = "name")]
    pub name: String,
    /// Name of parent catalog.
    #[serde(rename = "catalog_name")]
    pub catalog_name: String,
    /// Name of parent schema relative to its parent catalog.
    #[serde(rename = "schema_name")]
    pub schema_name: String,
    #[serde(rename = "table_type")]
    pub table_type: models::TableType,
    #[serde(rename = "data_source_format")]
    pub data_source_format: models::DataSourceFormat,
    /// The array of __ColumnInfo__ definitions of the table's columns.
    #[serde(rename = "columns")]
    pub columns: Vec<models::ColumnInfo>,
    /// Storage root URL for table (for **MANAGED**, **EXTERNAL** tables)
    #[serde(rename = "storage_location", skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    /// User-provided free-form text description.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// A map of key-value properties attached to the securable.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
}

impl CreateTable {
    pub fn new(name: String, catalog_name: String, schema_name: String, table_type: models::TableType, data_source_format: models::DataSourceFormat, columns: Vec<models::ColumnInfo>) -> CreateTable {
        CreateTable {
            name,
            catalog_name,
            schema_name,
            table_type,
            data_source_format,
            columns,
            storage_location: None,
            comment: None,
            properties: None,
        }
    }
}

