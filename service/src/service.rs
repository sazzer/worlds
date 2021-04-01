#[cfg(test)]
pub mod testing;

use crate::settings::Settings;
use prometheus::Registry;

/// The complete New Landing service.
pub struct Service {
    /// The HTTP Server.
    server: crate::server::Server,
}

impl Service {
    /// Construct a new instance of the service.
    ///
    /// # Parameters
    /// - `cfg` - The configuration settings for the service
    ///
    /// # Returns
    /// The service itself.
    pub async fn new(cfg: Settings) -> Self {
        tracing::debug!("Building Worlds");

        let prometheus = Registry::new();

        let authorization = crate::authorization::component::new("secret");
        let users = crate::users::component::new();
        let home = crate::home::component::new().with_contributor(users.home_links.clone()).build();

        let server = crate::server::component::new()
            .with_routes(authorization)
            .with_routes(home)
            .build(cfg.port, prometheus);

        tracing::debug!("Built Worlds");

        Self { server: server.server }
    }

    /// Start the service running.
    pub async fn start(self) {
        tracing::info!("Starting Worlds");
        self.server.start().await;
    }
}
