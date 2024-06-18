use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCredentials {
    /// The access key ID that identifies the temporary credentials.
    pub access_key_id: String,
    /// The secret access key that can be used to sign AWS API requests.
    pub secret_access_key: String,
    /// The token that users must pass to AWS API to use the temporary credentials.
    pub session_token: String,
}
