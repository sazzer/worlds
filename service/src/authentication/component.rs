use std::sync::Arc;

use actix_web::web::{post, resource, ServiceConfig};

use super::AuthenticationService;
use crate::{authorization::AuthorizationService, server::RouteConfigurer, users::UserService};

/// Component for authentication.
pub struct Component {
    service: Arc<AuthenticationService>,
}

impl Component {
    /// Create a new authentication component.
    pub fn new(users_service: Arc<UserService>, authorization_service: Arc<AuthorizationService>) -> Arc<Self> {
        let service = Arc::new(AuthenticationService::new(users_service, authorization_service));

        Arc::new(Self { service })
    }
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone());
        config.service(resource("/authenticate/check").route(post().to(super::endpoints::check::handle)));
        config.service(resource("/authenticate/authenticate").route(post().to(super::endpoints::authenticate::handle)));
    }
}
