use actix_web::web::ServiceConfig;

use crate::server::RouteConfigurer;

use super::usecases::AuthorizeSecurityContextUseCase;
use std::sync::Arc;

/// Component representig authorization of requests.
pub struct Component {
    authorizer: Arc<AuthorizeSecurityContextUseCase>,
}

/// Create a new instance of the authorization component.
pub fn new<S>(secret: S) -> Arc<Component>
where
    S: Into<String>,
{
    let authorizer = AuthorizeSecurityContextUseCase::new(&secret.into());

    let component = Component {
        authorizer: Arc::new(authorizer),
    };

    Arc::new(component)
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.data(self.authorizer.clone());
    }
}
