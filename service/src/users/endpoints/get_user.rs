use super::model::UserModel;
use crate::{
    http::{
        problem::{Problem, NOT_FOUND},
        response::{Response, SimpleRespondable},
    },
    users::{UserId, UserService},
};
use actix_web::web::{Data, Path};
use std::sync::Arc;

pub async fn handle(
    service: Data<Arc<UserService>>,
    path: Path<String>,
) -> Result<Response<SimpleRespondable<UserModel>>, Problem> {
    let user_id: UserId = path.parse().map_err(|e| {
        tracing::warn!(e = ?e, path = ?path, "Failed to parse User ID");

        Problem::from(NOT_FOUND)
    })?;

    service
        .get_user_by_id(user_id)
        .await
        .ok_or_else(|| NOT_FOUND.into())
        .map(|user| user.into())
}
