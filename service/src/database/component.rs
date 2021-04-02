use super::Database;
use std::sync::Arc;

/// Component for the database connection.
pub struct Component {
    database: Arc<Database>,
}

/// Create a new database component.
pub async fn new(url: &str) -> Component {
    let db = Database::new(url).await;

    Component { database: Arc::new(db) }
}
