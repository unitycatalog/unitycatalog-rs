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
pub struct ListCatalogsResponse {
    /// An array of catalog information objects.
    #[serde(rename = "catalogs", skip_serializing_if = "Option::is_none")]
    pub catalogs: Option<Vec<models::CatalogInfo>>,
    /// Opaque token to retrieve the next page of results. Absent if there are no more pages. __page_token__ should be set to this value for the next request (for the next page of results). 
    #[serde(rename = "next_page_token", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl ListCatalogsResponse {
    pub fn new() -> ListCatalogsResponse {
        ListCatalogsResponse {
            catalogs: None,
            next_page_token: None,
        }
    }
}

