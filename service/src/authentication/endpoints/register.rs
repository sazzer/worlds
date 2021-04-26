use std::sync::Arc;

use actix_web::web::{Data, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use super::model::AuthenticatedModel;
use crate::{
    authentication::AuthenticationService,
    http::{
        problem::Problem,
        valid::{Valid, Validatable},
    },
    users::{Email, Username},
};

/// Handle the authentication request.
pub async fn handle(_req: Valid<RegisterRequest>, _service: Data<Arc<AuthenticationService>>) -> Result<Json<AuthenticatedModel>, Problem> {
    todo!()
}

/// The incoming request to authenticate.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub username:     Username,
    pub email:        Email,
    pub display_name: String,
    pub password:     String,
}

impl Validatable for RegisterRequest {
    fn schema() -> Value {
        json!({
            "type": "object",
            "properties": {
                "username": Username::schema(),
                "email": Email::schema(),
                "displayName": {
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
                "email",
                "displayName",
                "password"
            ]
        })
    }
}
