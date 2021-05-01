use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::authorization::AccessToken;

/// Model to return if authentication was a success
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedModel {
    pub token:      AccessToken,
    pub user_id:    Option<String>,
    pub expires_at: DateTime<Utc>,
}
