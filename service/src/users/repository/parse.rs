use tokio_postgres::Row;

use crate::{
    model::Identity,
    users::{UserData, UserResource},
};

impl From<Row> for UserResource {
    fn from(row: Row) -> Self {
        UserResource {
            identity: Identity {
                id:      row.get("user_id"),
                version: row.get("version"),
                created: row.get("created"),
                updated: row.get("updated"),
            },
            data:     UserData {
                username:     row.get("username"),
                email:        row.get("email"),
                display_name: row.get("display_name"),
                password:     row.get("password"),
            },
        }
    }
}
