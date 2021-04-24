use std::sync::Arc;

use crate::server::RouteConfigurer;
use actix_web::web::{post, resource, ServiceConfig};

/// Component for authentication.
pub struct Component {}

impl Component {
    /// Create a new authentication component.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.service(
            resource("/authenticate/check").route(post().to(super::endpoints::check::handle)),
        );
        config.service(
            resource("/authenticate/authenticate")
                .route(post().to(super::endpoints::authenticate::handle)),
        );
    }
}
