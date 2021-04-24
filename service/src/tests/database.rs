mod postgres;
pub mod seed;

use std::str::FromStr;

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use lazy_static::lazy_static;
use postgres::Postgres;
use testcontainers::{clients::Cli, Container, Docker};

lazy_static! {
    static ref DOCKER: Cli = Cli::default();
}

/// Wrapper around a Postgres container for testing.
pub struct TestDatabase {
    #[allow(dead_code)]
    node:     Container<'static, Cli, Postgres>,
    pub host: String,
    pub port: u16,
    pub url:  String,
}

impl TestDatabase {
    /// Create a new test database.
    #[allow(clippy::new_without_default)] // This is a non-trivial implementation. Hiding it behind default() is unwise.
    pub async fn new() -> Self {
        tracing::info!("Starting Postgres database");
        let node = DOCKER.run(Postgres::default());

        let host = std::env::var("DOCKER_HOSTNAME").unwrap_or_else(|_| "localhost".to_owned());
        let port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres@{}:{}", host, port);
        tracing::info!(url = ?url, "Running postgres");

        Self { node, host, port, url }
    }

    /// Seed some data into the database
    ///
    /// # Parameters
    /// - `data` - The data to seed
    pub async fn seed(&self, data: &dyn seed::SeedData) {
        tracing::debug!(data = ?data, "Seeding data");

        let pg_config = tokio_postgres::Config::from_str(&self.url).expect("Invalid database URL");

        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        let pool = Pool::new(mgr, 16);

        let conn = pool.get().await.expect("Failed to get database connection");

        conn.execute(data.sql(), &data.binds()[..]).await.unwrap();
    }
}
