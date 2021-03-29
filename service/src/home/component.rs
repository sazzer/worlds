use super::{HomeLinksUseCase, LinkContributor};
use crate::server::RouteConfigurer;
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// Component for the home document.
pub struct Component {
    service: Arc<HomeLinksUseCase>,
}

/// Builder for building the home document component.
#[derive(Default)]
pub struct Builder {
    contributors: Vec<Arc<dyn LinkContributor>>,
}

/// Create a new instance of the home document builder.
pub fn new() -> Builder {
    Builder::default().with_contributor(Arc::new(vec![("self".to_owned(), "/".into())]))
}

impl Builder {
    /// Add a new contributor of links to the home document.
    ///
    /// # Parameters
    /// - `contributor` - The contributor that can add to the home document
    #[allow(dead_code)]
    pub fn with_contributor(mut self, contributor: Arc<dyn LinkContributor>) -> Self {
        self.contributors.push(contributor);

        self
    }

    /// Build the actual home document component.
    pub fn build(self) -> Arc<Component> {
        let service = Arc::new(HomeLinksUseCase {
            contributors: self.contributors,
        });

        Arc::new(Component { service })
    }
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone());
        super::http::configure_routes(config);
    }
}
