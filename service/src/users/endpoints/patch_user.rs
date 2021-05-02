use std::sync::Arc;

use actix_web::web::{Data, Path};
use serde::Deserialize;
use serde_json::{json, Value};

use super::model::FullUserResponse;
use crate::{
    authorization::{Authentication, Principal},
    http::{
        problem::{Problem, FORBIDDEN, NOT_FOUND},
        valid::{Valid, Validatable},
    },
    users::{Email, Password, UserData, UserId, UserService},
};

pub async fn handle(
    service: Data<Arc<UserService>>,
    path: Path<String>,
    request: Valid<PatchRequest>,
    authentication: Authentication,
) -> Result<FullUserResponse, Problem> {
    let user_id: UserId = path.parse().map_err(|e| {
        tracing::warn!(e = ?e, path = ?path, "Failed to parse User ID");

        FORBIDDEN
    })?;

    authentication.same_principal(&Principal::from(&user_id))?;

    let request = request.unwrap();

    let user = service
        .update_user_by_id(&user_id, move |user| {
            if let Some(old_password) = request.old_password {
                if user.password != &old_password {
                    todo!("Return correct error");
                    return Err(NOT_FOUND);
                }
            }

            Ok(UserData {
                email: request.email.unwrap_or(user.email),
                display_name: request.display_name.unwrap_or(user.display_name),
                password: request.password.map_or(user.password, |p| Password::from_plaintext(&p)),
                ..user
            })
        })
        .await
        .map_err(|_e| NOT_FOUND)?;

    Ok(user.into())
}

/// The incoming request to patch user details.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchRequest {
    pub email:        Option<Email>,
    pub display_name: Option<String>,
    pub password:     Option<String>,
    pub old_password: Option<String>,
}

impl Validatable for PatchRequest {
    fn schema() -> Value {
        json!({
            "type": "object",
            "properties": {
                "email": Email::schema(),
                "displayName": {
                    "type": "string",
                    "minLength": 1
                },
                "password": {
                    "type": "string",
                    "minLength": 1
                },
                "oldPassword": {
                    "type": "string",
                    "minLength": 1
                }
            },
            "if": {
                "required": ["password"]
            },
            "then": {
                "required": ["oldPassword"]
            }
        })
    }
}
