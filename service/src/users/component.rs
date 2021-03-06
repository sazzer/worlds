use std::sync::Arc;

use actix_web::web::{get, patch, resource, ServiceConfig};

use super::{repository::UserRepository, service::UserService};
use crate::{database::Database, server::RouteConfigurer};

/// Component for working with user records.
pub struct Component {
    pub service: Arc<UserService>,
}

impl Component {
    /// Create a new users component.
    pub fn new(database: Arc<Database>) -> Arc<Self> {
        let repository = UserRepository::new(database);
        let service = Arc::new(UserService::new(repository));

        Arc::new(Self { service })
    }
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone());

        config.service(
            resource("/users/{id}")
                .route(get().to(super::endpoints::get_user::handle))
                .route(patch().to(super::endpoints::patch_user::handle)),
        );
    }
}
