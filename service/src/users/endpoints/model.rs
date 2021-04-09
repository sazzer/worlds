use serde::Serialize;

/// Representation of a user on the HTTP API.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
    pub username: String,
    pub email: String,
    pub display_name: String,
}
