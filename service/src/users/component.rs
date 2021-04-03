use actix_web::web::ServiceConfig;

use super::{home::HomeLinks, repository::Repository, GetUserUseCase};
use crate::{database::Database, home::LinkContributor, server::RouteConfigurer};
use std::sync::Arc;

/// Component for working with users.
pub struct Component {
    pub home_links: Arc<dyn LinkContributor>,
    pub get_user: Arc<GetUserUseCase>,
}

/// Create a new instance of the users component.
pub fn new(database: Arc<Database>) -> Arc<Component> {
    let repository = Arc::new(Repository::new(database));
    let get_user = Arc::new(GetUserUseCase::new(repository));

    let home_links = Arc::new(HomeLinks {});

    Arc::new(Component { home_links, get_user })
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.data(self.get_user.clone());
    }
}
