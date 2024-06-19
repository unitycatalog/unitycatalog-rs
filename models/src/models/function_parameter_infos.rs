use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionParameterInfos {
    /// The array of __FunctionParameterInfo__ definitions of the function's parameters.
    pub parameters: Vec<models::FunctionParameterInfo>,
}
