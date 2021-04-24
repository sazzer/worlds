use std::sync::Arc;

use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    http::{
        problem::Problem,
        valid::{Valid, Validatable},
    },
    users::{UserService, Username},
};

/// Handle the authentication request.
pub async fn handle(service: Data<Arc<UserService>>, req: Valid<CheckRequest>) -> Result<Json<CheckModel>, Problem> {
    let user = service.get_user_by_username(&req.username).await;

    Ok(Json(CheckModel { known: user.is_some() }))
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
