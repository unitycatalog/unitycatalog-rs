use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFunctionRequest {
    pub function_info: models::CreateFunction,
}
