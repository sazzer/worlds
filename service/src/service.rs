#[cfg(test)]
pub mod testing;

use std::sync::Arc;

use crate::{server::Server, settings::Settings};

/// The actual service.
pub struct Service {
    server:                Server,
    authorization_service: Arc<crate::authorization::AuthorizationService>,
}

impl Service {
    /// Create a new instance of the service.
    #[tracing::instrument]
    pub async fn new(settings: Settings) -> Self {
        tracing::info!("Building Worlds");

        let db = crate::database::component::Component::new(&settings.database_url).await;
        let authorization = crate::authorization::component::Component::new("secret");
        let users = crate::users::component::Component::new(db.database);
        let authentication = crate::authentication::component::Component::new(users.service.clone(), authorization.service.clone());

        let server = crate::server::component::Builder::default()
            .with_routes(authorization.clone())
            .with_routes(authentication)
            .with_routes(users)
            .build(settings.port);

        tracing::info!("Built Worlds");
        Self {
            server:                server.server,
            authorization_service: authorization.service.clone(),
        }
    }

    /// Start the service running.
    pub async fn start(self) {
        tracing::info!("Starting Worlds");
        self.server.start().await;
    }
}
