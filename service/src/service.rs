#[cfg(test)]
pub mod testing;

use crate::server::Server;
use crate::settings::Settings;

/// The actual service.
pub struct Service {
    server: Server,
}

impl Service {
    /// Create a new instance of the service.
    #[tracing::instrument]
    pub async fn new(settings: Settings) -> Self {
        tracing::info!("Building Worlds");

        let db = crate::database::component::Component::new(&settings.database_url).await;
        let users = crate::users::component::Component::new(db.database);

        let server = crate::server::component::Builder::default()
            .with_routes(users)
            .build(settings.port);

        tracing::info!("Built Worlds");
        Self {
            server: server.server,
        }
    }

    /// Start the service running.
    pub async fn start(self) {
        tracing::info!("Starting Worlds");
        self.server.start().await;
    }
}
