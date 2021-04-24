use std::sync::Arc;

use actix_web::{
    web::{Data, Path},
    Either,
};

use super::model::{FullUserResponse, SimpleUserResponse};
use crate::{
    authorization::{Authentication, Principal},
    http::problem::{Problem, NOT_FOUND},
    users::{UserId, UserService},
};

pub async fn handle(
    service: Data<Arc<UserService>>,
    path: Path<String>,
    authentication: Authentication,
) -> Result<Either<FullUserResponse, SimpleUserResponse>, Problem> {
    let user_id: UserId = path.parse().map_err(|e| {
        tracing::warn!(e = ?e, path = ?path, "Failed to parse User ID");

        NOT_FOUND
    })?;

    let user = service.get_user_by_id(&user_id).await.ok_or(NOT_FOUND)?;

    if authentication.principal().filter(|p| p == &&Principal::from(user_id)).is_some() {
        Ok(Either::Left(user.into()))
    } else {
        Ok(Either::Right(user.into()))
    }
}
