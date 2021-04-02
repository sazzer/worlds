use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::str::FromStr;

/// Wrapper around a database connection pool
pub struct Database {
    pool: Pool,
}

impl Database {
    /// Create a new database wrapper.
    ///
    /// # Parameters
    /// - `url` - The URL to connect to.
    pub async fn new(url: &str) -> Self {
        let pg_config = tokio_postgres::Config::from_str(url).expect("Invalid database URL");

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        let pool = Pool::new(mgr, 16);

        pool.get().await.expect("Unable to open database connection");

        Self { pool }
    }
}
