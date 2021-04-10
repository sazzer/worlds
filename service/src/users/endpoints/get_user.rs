use super::model::UserModel;
use crate::{
    http::{
        problem::{Problem, NOT_FOUND},
        response::{Response, SimpleRespondable},
    },
    model::Identity,
    users::{UserData, UserResource},
};
pub async fn handle() -> Result<Response<SimpleRespondable<UserModel>>, Problem> {
    let user = Some(UserResource {
        identity: Identity::default(),
        data: UserData {
            username: "sazzer".parse().unwrap(),
            email: "graham@grahamcox.co.uk".parse().unwrap(),
            display_name: "Graham".to_owned(),
        },
    });

    user.ok_or_else(|| NOT_FOUND.into()).map(|user| user.into())
}
