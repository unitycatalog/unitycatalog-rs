use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFunctionsResponse {
    /// An array of function information objects.
    pub functions: Vec<models::FunctionInfo>,
    /// Opaque token to retrieve the next page of results. Absent if there are no more pages. __page_token__ should be set to this value for the next request (for the next page of results).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}
