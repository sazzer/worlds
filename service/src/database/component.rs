use super::Database;
use std::sync::Arc;

/// Component for the database connection.
pub struct Component {
    database: Arc<Database>,
}

impl Component {
    /// Create a new database component.
    pub async fn new(url: &str) -> Self {
        let db = Arc::new(Database::new(url).await);

        let conn = db.connect().await;

        Self { database: db }
    }
}
