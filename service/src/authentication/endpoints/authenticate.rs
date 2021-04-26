use std::sync::Arc;

use actix_web::web::{Data, Json};
use serde::Deserialize;
use serde_json::{json, Value};

use super::model::AuthenticatedModel;
use crate::{
    authentication::AuthenticationService,
    authorization::Principal,
    http::{
        problem::{Problem, UNAUTHORIZED},
        valid::{Valid, Validatable},
    },
    users::Username,
};

/// Handle the authentication request.
pub async fn handle(
    req: Valid<AuthenticateRequest>,
    service: Data<Arc<AuthenticationService>>,
) -> Result<Json<AuthenticatedModel>, Problem> {
    let (security_context, token) = service.authenticate(&req.username, &req.password).await.map_err(|e| {
        tracing::warn!(username = ?req.username, e = ?e, "Authentication failed");

        UNAUTHORIZED
    })?;

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
pub struct AuthenticateRequest {
    pub username: Username,
    pub password: String,
}

impl Validatable for AuthenticateRequest {
    fn schema() -> Value {
        json!({
            "type": "object",
            "properties": {
                "username": Username::schema(),
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
