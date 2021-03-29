pub mod component;
mod span;

use actix_cors::Cors;
use actix_http::http::header;
use actix_web::{middleware::Logger, web::ServiceConfig, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use std::sync::Arc;

/// The HTTP Server running the application.
pub struct Server {
    port: u16,
    prometheus: prometheus::Registry,
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
    /// Start the server listening on the configured port.
    pub async fn start(self) {
        let address = format!("0.0.0.0:{}", self.port);

        tracing::debug!(address = ?address, "Starting HTTP server");

        let prometheus =
            PrometheusMetrics::new_with_registry(self.prometheus, "actix", Some("/metrics"), None)
                .unwrap();
        let routes = self.routes.clone();

        HttpServer::new(move || {
            let prometheus = prometheus.clone();
            let routes = routes.clone();

            let mut app = App::new()
                .wrap(prometheus)
                .wrap(Logger::default())
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .expose_headers(vec![header::ETAG, header::LOCATION, header::LINK]),
                )
                .wrap(span::Span);

            for c in &routes {
                app = app.configure(move |server_config| {
                    c.configure_routes(server_config);
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
