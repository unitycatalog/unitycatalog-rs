use crate::models;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateTemporaryTableCredentialResponse {
    pub aws_temp_credentials: models::AwsCredentials,
    /// Server time when the credential will expire, in epoch milliseconds. The API client is advised to cache the credential given this expiration time.
    #[serde(with = "ts_milliseconds")]
    pub expiration_time: DateTime<Utc>,
}
