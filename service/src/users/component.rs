use std::sync::Arc;

use actix_web::web::{get, resource, ServiceConfig};

use crate::{database::Database, server::RouteConfigurer};

use super::{repository::UserRepository, service::UserService};

/// Component for working with user records.
pub struct Component {
    service: Arc<UserService>,
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

        config.service(resource("/users/{id}").route(get().to(super::endpoints::get_user::handle)));
    }
}
