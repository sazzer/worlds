use std::sync::Arc;

use actix_web::web::{Data, Path};

use super::model::FullUserResponse;
use crate::{
    authorization::{Authentication, Principal},
    http::problem::{Problem, FORBIDDEN, NOT_FOUND},
    users::{UserId, UserService},
};

pub async fn handle(
    service: Data<Arc<UserService>>,
    path: Path<String>,
    authentication: Authentication,
) -> Result<FullUserResponse, Problem> {
    let user_id: UserId = path.parse().map_err(|e| {
        tracing::warn!(e = ?e, path = ?path, "Failed to parse User ID");

        FORBIDDEN
    })?;

    authentication.same_principal(&Principal::from(&user_id))?;

    let user = service.get_user_by_id(&user_id).await.ok_or(NOT_FOUND)?;

    Ok(user.into())
}
