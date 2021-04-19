use std::sync::Arc;

use crate::server::RouteConfigurer;
use actix_web::web::ServiceConfig;

/// Component for authentication.
pub struct Component {}

impl Component {
    /// Create a new authentication component.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, _config: &mut ServiceConfig) {}
}
