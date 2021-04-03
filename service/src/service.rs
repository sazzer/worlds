#[cfg(test)]
pub mod testing;

use crate::settings::Settings;
use prometheus::Registry;
use std::sync::Arc;

/// The complete New Landing service.
pub struct Service {
    /// The HTTP Server.
    server: crate::server::Server,
    #[allow(dead_code)] // Used for integration tests.
    access_token_generator: Arc<crate::authorization::GenerateSecurityContextUseCase>,
}

impl Service {
    /// Construct a new instance of the service.
    ///
    /// # Parameters
    /// - `cfg` - The configuration settings for the service
    ///
    /// # Returns
    /// The service itself.
    #[tracing::instrument]
    pub async fn new(cfg: Settings) -> Self {
        tracing::debug!("Building Worlds");

        let prometheus = Registry::new();

        let database = crate::database::component::new(&cfg.database_url).await;
        let authorization = crate::authorization::component::new("secret");
        let users = crate::users::component::new(database.database);
        let home = crate::home::component::new().with_contributor(users.home_links.clone()).build();

        let server = crate::server::component::new()
            .with_routes(authorization.clone())
            .with_routes(home)
            .with_routes(users)
            .build(cfg.port, prometheus);

        tracing::debug!("Built Worlds");

        Self {
            server: server.server,
            access_token_generator: authorization.generator.clone(),
        }
    }

    /// Start the service running.
    pub async fn start(self) {
        tracing::info!("Starting Worlds");
        self.server.start().await;
    }
}
