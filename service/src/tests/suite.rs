use super::database::TestDatabase;
use crate::{
    service::{testing::TestResponse, Service},
    settings::Settings,
};
use actix_http::Request;

/// Wrapper around the components needed to test the service.
pub struct TestSuite {
    #[allow(dead_code)]
    db: TestDatabase,

    service: Service,
}

impl TestSuite {
    /// Create a new test suite.
    pub async fn new() -> Self {
        let _ = env_logger::try_init();

        let db = TestDatabase::new().await;

        let service = Service::new(Settings {
            port: 0,
            database_url: db.url.clone(),
        })
        .await;

        Self { db, service }
    }

    /// Inject a request into the service and return the response.
    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}