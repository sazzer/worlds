pub mod component;

use actix_cors::Cors;
use actix_http::http::header;
use actix_web::{middleware::Logger, App, HttpServer};

/// The HTTP Server.
pub struct Server {
    port: u16,
}

impl Server {
    /// Create a new instance of the HTTP Server.
    pub(self) fn new(port: u16) -> Self {
        Self { port }
    }

    /// Start the server listening.
    pub async fn start(self) {
        let address = format!("0.0.0.0:{}", self.port);

        tracing::info!(address = ?address, "Starting HTTP Server");

        HttpServer::new(move || {
            let app = App::new().wrap(Logger::default()).wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .expose_headers(vec![header::ETAG, header::LOCATION, header::LINK]),
            );

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
