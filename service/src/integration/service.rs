use crate::service::{testing::TestResponse, Service};
use actix_http::Request;

pub struct TestService {
    service: Service,
}

impl TestService {
    pub async fn new() -> Self {
        let _ = env_logger::try_init();

        let cfg = crate::settings::Settings { port: 0 };

        let service = Service::new(cfg).await;
        Self { service }
    }

    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}
