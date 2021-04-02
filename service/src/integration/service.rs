use crate::service::{testing::TestResponse, Service};
use crate::testdatabase::TestDatabase;
use actix_http::{
    error::ParseError,
    http::{
        header::{Header, IntoHeaderValue, InvalidHeaderValue, AUTHORIZATION},
        HeaderName, HeaderValue,
    },
    HttpMessage, Request,
};

/// Wrapper around the service being tested.
pub struct TestService {
    service: Service,
    #[allow(dead_code)] // Used for RAII purposes.
    test_database: TestDatabase,
}

impl TestService {
    pub async fn new() -> Self {
        let _ = env_logger::try_init();

        let test_database = TestDatabase::new();
        let cfg = crate::settings::Settings {
            port: 0,
            database_url: test_database.url.clone(),
        };

        let service = Service::new(cfg).await;
        Self { service, test_database }
    }

    pub fn authorization<S>(&self, user_id: S) -> AuthorizationHeader
    where
        S: Into<String>,
    {
        let user_id = user_id.into();
        let token = self.service.authorize_user(&user_id);

        AuthorizationHeader { token: token.0 }
    }

    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}

/// Representation of the Authorization header to use.
pub struct AuthorizationHeader {
    token: String,
}

impl Header for AuthorizationHeader {
    fn name() -> HeaderName {
        AUTHORIZATION
    }

    fn parse<T: HttpMessage>(_: &T) -> Result<Self, ParseError> {
        todo!()
    }
}

impl IntoHeaderValue for AuthorizationHeader {
    type Error = InvalidHeaderValue;

    fn try_into(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&format!("Bearer {}", self.token))
    }
}
