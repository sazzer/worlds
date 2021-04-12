use super::Database;
use std::sync::Arc;

/// Component for the database connection.
pub struct Component {
    pub database: Arc<Database>,
}

impl Component {
    /// Create a new database component.
    pub async fn new(url: &str) -> Self {
        let db = Arc::new(Database::new(url).await);

        super::migrate::migrate(&db).await;

        Self { database: db }
    }
}
