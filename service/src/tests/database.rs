mod postgres;

use lazy_static::lazy_static;
use postgres::Postgres;
use testcontainers::{clients::Cli, Container, Docker};

lazy_static! {
    static ref DOCKER: Cli = Cli::default();
}

/// Wrapper around a Postgres container for testing.
pub struct TestDatabase {
    #[allow(dead_code)]
    node: Container<'static, Cli, Postgres>,
    pub host: String,
    pub port: u16,
    pub url: String,
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

        Self {
            node,
            host,
            port,
            url,
        }
    }
}
