use actix_web::web::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    authorization::AccessToken,
    http::{
        problem::{Problem, UNAUTHORIZED},
        valid::{Valid, Validatable},
    },
    users::Username,
};

/// Handle the authentication request.
pub async fn handle(_req: Valid<AuthenticateRequest>) -> Result<Json<AuthenticatedModel>, Problem> {
    Err(Problem::from(UNAUTHORIZED))
}

/// The incoming request to authenticate.
#[derive(Deserialize)]
pub struct AuthenticateRequest {
    pub username: Username,
    pub password: String,
}

impl Validatable for AuthenticateRequest {
    fn schema() -> Value {
        json!({
            "type": "object",
            "properties": {
                "username": {
                    "type": "string",
                    "minLength": 1
                },
                "password": {
                    "type": "string",
                    "minLength": 1
                }
            },
            "required": [
                "username",
                "password"
            ]
        })
    }
}

/// Model to return if authentication was a success
#[derive(Debug, Serialize)]
pub struct AuthenticatedModel {
    pub token:      AccessToken,
    pub expires_at: DateTime<Utc>,
}
