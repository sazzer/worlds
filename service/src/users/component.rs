use std::sync::Arc;

use actix_web::web::{get, resource, ServiceConfig};

use crate::server::RouteConfigurer;

/// Component for working with user records.
pub struct Component {}

impl Component {
    /// Create a new users component.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {})
    }
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.service(resource("/users/{id}").route(get().to(super::endpoints::get_user::handle)));
    }
}
