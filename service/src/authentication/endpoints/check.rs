use crate::{
    http::{
        problem::Problem,
        valid::{Valid, Validatable},
    },
    users::Username,
};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Handle the authentication request.
pub async fn handle(_req: Valid<CheckRequest>) -> Result<Json<CheckModel>, Problem> {
    Ok(Json(CheckModel { known: false }))
}

/// The incoming request to check if a username is know or not.
#[derive(Deserialize)]
pub struct CheckRequest {
    pub username: Username,
}

impl Validatable for CheckRequest {
    fn schema() -> Value {
        json!({
            "type": "object",
            "properties": {
                "username": {
                    "type": "string",
                    "minLength": 1
                },
            },
            "required": [
                "username"
            ]
        })
    }
}

/// Model to return if authentication was a success
#[derive(Debug, Serialize)]
pub struct CheckModel {
    pub known: bool,
}
