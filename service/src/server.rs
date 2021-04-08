pub mod component;
mod span;

use std::sync::Arc;

use actix_cors::Cors;
use actix_http::http::header;
use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};

/// The HTTP Server.
pub struct Server {
    port: u16,

    pub(super) routes: Vec<Arc<dyn RouteConfigurer>>,
}

/// Trait that can be implemented by other components to configure routes into the HTTP Server.
pub trait RouteConfigurer: Send + Sync {
    /// Configure some routes onto the provided HTTP Server configuration.
    ///
    /// # Parameters
    /// - `config` - The HTTP Server configuration to wire the routes onto
    fn configure_routes(&self, config: &mut ServiceConfig);
}

impl Server {
    /// Create a new instance of the HTTP Server.
    pub(self) fn new(port: u16, routes: Vec<Arc<dyn RouteConfigurer>>) -> Self {
        Self { port, routes }
    }

    /// Start the server listening.
    pub async fn start(self) {
        let address = format!("0.0.0.0:{}", self.port);

        tracing::info!(address = ?address, "Starting HTTP Server");

        let routes = self.routes.clone();

        HttpServer::new(move || {
            let routes = routes.clone();

            let mut app = App::new()
                .wrap(Logger::default())
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .expose_headers(vec![header::ETAG, header::LOCATION, header::LINK]),
                )
                .wrap(span::Span);

            for r in &routes {
                app = app.configure(move |server_config| {
                    r.configure_routes(server_config);
                });
            }

            tracing::trace!("Built listener");

            app
        })
        .bind(address)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}
