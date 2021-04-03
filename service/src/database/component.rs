use super::Database;
use std::sync::Arc;

/// Component for the database connection.
pub struct Component {
    pub database: Arc<Database>,
}

/// Create a new database component.
pub async fn new(url: &str) -> Component {
    let db = Database::new(url).await;

    super::migrate::migrate(&db).await;

    Component { database: Arc::new(db) }
}
