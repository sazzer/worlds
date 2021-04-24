use std::sync::Arc;

use actix_web::web::ServiceConfig;

use super::service::AuthorizationService;
use crate::server::RouteConfigurer;

/// Component for authorization.
pub struct Component {
    /// The authorization service
    pub service: Arc<AuthorizationService>,
}

impl Component {
    /// Create a new authorization component.
    pub fn new(secret: &str) -> Arc<Self> {
        let service = Arc::new(AuthorizationService::new(secret));
        Arc::new(Self { service })
    }
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone());
    }
}
