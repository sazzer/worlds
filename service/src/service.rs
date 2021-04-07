/// The actual service.
pub struct Service {}

impl Service {
    /// Create a new instance of the service.
    #[tracing::instrument]
    pub async fn new() -> Self {
        tracing::info!("Building Worlds");

        tracing::info!("Built Worlds");
        Self {}
    }

    /// Start the service running.
    pub async fn start(self) {
        tracing::info!("Starting Worlds");
    }
}
