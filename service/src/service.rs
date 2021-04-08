use crate::server::Server;

/// The actual service.
pub struct Service {
    server: Server,
}

impl Service {
    /// Create a new instance of the service.
    #[tracing::instrument]
    pub async fn new() -> Self {
        tracing::info!("Building Worlds");

        let users = crate::users::component::Component::new();

        let server = crate::server::component::Builder::default()
            .with_routes(users)
            .build(8000);

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
