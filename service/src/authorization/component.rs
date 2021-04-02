use actix_web::web::ServiceConfig;
use chrono::Duration;

use crate::server::RouteConfigurer;

use super::{usecases::AuthorizeSecurityContextUseCase, GenerateSecurityContextUseCase};
use std::sync::Arc;

/// Component representig authorization of requests.
pub struct Component {
    authorizer: Arc<AuthorizeSecurityContextUseCase>,
    pub generator: Arc<GenerateSecurityContextUseCase>,
}

/// Create a new instance of the authorization component.
pub fn new<S>(secret: S) -> Arc<Component>
where
    S: Into<String>,
{
    let secret = secret.into();

    let authorizer = AuthorizeSecurityContextUseCase::new(&secret);
    let generator = GenerateSecurityContextUseCase::new(&secret, Duration::days(365));

    let component = Component {
        authorizer: Arc::new(authorizer),
        generator: Arc::new(generator),
    };

    Arc::new(component)
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.data(self.authorizer.clone());
    }
}
