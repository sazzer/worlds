use super::{RouteConfigurer, Server};
use std::sync::Arc;

/// Builder for the HTTP Server component.
#[derive(Default)]
pub struct Builder {
    routes: Vec<Arc<dyn RouteConfigurer>>,
}

/// The HTTP Server component.
pub struct Component {
    pub server: Server,
}

impl Builder {
    /// Add new routes to the server.
    pub fn with_routes(mut self, route: Arc<dyn RouteConfigurer>) -> Self {
        self.routes.push(route);
        self
    }

    /// Build the HTTP Server component.
    pub fn build(self, port: u16) -> Component {
        Component {
            server: Server::new(port, self.routes),
        }
    }
}
