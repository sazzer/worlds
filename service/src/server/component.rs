use super::{RouteConfigurer, Server};
use std::sync::Arc;

/// Component representing the HTTP Server.
pub struct Component {
    pub server: Server,
}

/// Builder to help build the `Component`.
#[derive(Default)]
pub struct Builder {
    routes: Vec<Arc<dyn RouteConfigurer>>,
}

/// Create a new builder to build the component with.
pub fn new() -> Builder {
    Builder::default()
}

impl Builder {
    /// Register a new `RouteConfigurer` that can contribute routes to the HTTP Server
    ///
    /// # Parameters
    /// - `routes` - The configurer for the routes to add
    pub fn with_routes(mut self, routes: Arc<dyn RouteConfigurer>) -> Self {
        self.routes.push(routes);

        self
    }

    /// Actually build the HTTP Server component
    ///
    /// # Parameters
    /// - `port` - The port to listen on
    /// - `prometheus` - The prometheus registry to use
    pub fn build(self, port: u16, prometheus: prometheus::Registry) -> Component {
        tracing::debug!("Building HTTP Server component");
        Component {
            server: Server {
                port,
                prometheus,
                routes: self.routes,
            },
        }
    }
}
