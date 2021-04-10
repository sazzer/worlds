use super::model::UserModel;
use crate::http::{
    problem::{Problem, NOT_FOUND},
    response::{Response, SimpleRespondable},
};
pub async fn handle() -> Result<Response<SimpleRespondable<UserModel>>, Problem> {
    Err(NOT_FOUND.into())
}
