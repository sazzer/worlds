use crate::{
    authorization::AccessToken,
    http::problem::{Problem, UNAUTHORIZED},
};
use actix_web::web::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Handle the authentication request.
pub async fn handle() -> Result<Json<AuthenticatedModel>, Problem> {
    Err(Problem::from(UNAUTHORIZED))
}

/// Model to return if authentication was a success
#[derive(Debug, Serialize)]
pub struct AuthenticatedModel {
    pub token: AccessToken,
    pub expires_at: DateTime<Utc>,
}
