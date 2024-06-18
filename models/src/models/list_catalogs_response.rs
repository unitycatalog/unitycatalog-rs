use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListCatalogsResponse {
    /// An array of catalog information objects.
    pub catalogs: Vec<models::CatalogInfo>,
    /// Opaque token to retrieve the next page of results. Absent if there are no more pages. __page_token__ should be set to this value for the next request (for the next page of results).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}
