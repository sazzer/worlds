use std::sync::Arc;

use actix_web::web::{Data, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use super::model::AuthenticatedModel;
use crate::{
    authentication::{AuthenticationService, Registration},
    authorization::Principal,
    http::{
        problem::Problem,
        valid::{Valid, Validatable},
    },
    users::{Email, Password, Username},
};

/// Handle the authentication request.
pub async fn handle(req: Valid<RegisterRequest>, service: Data<Arc<AuthenticationService>>) -> Result<Json<AuthenticatedModel>, Problem> {
    let req = req.unwrap();

    let (security_context, token) = service
        .register(Registration {
            username:     req.username,
            email:        req.email,
            display_name: req.display_name,
            password:     Password::from_plaintext(&req.password),
        })
        .await
        .unwrap();

    Ok(Json(AuthenticatedModel {
        token,
        user_id: match security_context.principal {
            Principal::User(user_id) => Some(user_id),
        },
        expires_at: security_context.expires,
    }))
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
