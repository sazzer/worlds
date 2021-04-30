use std::{cmp::Ordering, ops::Deref, pin::Pin};

use actix_http::{http::StatusCode, Payload};
use actix_web::{web::Json, FromRequest, HttpRequest};
use futures::Future;
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::problem::SimpleProblemType;
use crate::http::problem::{Problem, BAD_REQUEST, UNPROCESSABLE_ENTITY};

/// Trait implemented by types that can be validated
pub trait Validatable {
    /// Generate the schema needed to validate this type.
    fn schema() -> Value;
}

/// Wrapper around some type that will perform JSON Schema validation when parsing from the HTTP
/// request.
pub struct Valid<T>(T)
where
    T: DeserializeOwned + Validatable;

impl<T> Valid<T>
where
    T: DeserializeOwned + Validatable,
{
    /// Unwrap the validated data, returning it without the wrapper.
    pub fn unwrap(self) -> T {
        self.0
    }
}

impl<T> FromRequest for Valid<T>
where
    T: DeserializeOwned + Validatable,
{
    type Config = ();
    type Error = Problem;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let value = Json::<Value>::from_request(req, payload);

        Box::pin(async move {
            // First, parse the request as arbitrary JSON.
            let value = value
                .await
                .map_err(|e| {
                    tracing::warn!(e = ?e, "Failed to parse request as JSON");

                    BAD_REQUEST
                })?
                .0;

            // Validate the JSON against the appropriate schema.
            let mut scope = valico::json_schema::Scope::new();
            let schema = scope.compile_and_return(T::schema(), false).unwrap();
            let validation = schema.validate(&value);

            if !validation.is_valid() {
                let mut errors = validation.errors;
                errors.sort_by(|a, b| {
                    let path = a.get_path().partial_cmp(b.get_path());
                    let code = a.get_code().partial_cmp(b.get_code());

                    match (path, code) {
                        (Some(Ordering::Equal), Some(c)) => c,
                        (Some(p), _) => p,
                        _ => Ordering::Equal,
                    }
                });

                return Err(Problem::from(VALIDATION_ERROR).with_extra("validationErrors", errors));
            }

            // Then attempt to parse the JSON into the target type.
            let result: T = serde_json::from_value(value).map_err(|e| {
                tracing::warn!(e = ?e, "Failed to parse JSON as correct type");

                UNPROCESSABLE_ENTITY
            })?;

            Ok(Valid(result))
        })
    }
}

impl<T> Deref for Valid<T>
where
    T: DeserializeOwned + Validatable,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Problem to indicate that a request failed validation.
const VALIDATION_ERROR: SimpleProblemType = SimpleProblemType {
    problem_type:  "tag:worlds,2021:problems/validation",
    problem_title: "Request body failed validation",
    status_code:   StatusCode::UNPROCESSABLE_ENTITY,
};

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use assert2::check;
    use insta::assert_json_snapshot;
    use serde::Deserialize;
    use serde_json::json;

    use super::*;

    #[derive(Deserialize)]
    pub struct Input {
        pub name: String,
    }

    impl Validatable for Input {
        fn schema() -> Value {
            json!({
              "type": "object",
              "properties": {
                "name": {
                  "type": "string",
                  "minLength": 1,
                  "maxLength": 5
                }
              },
              "required": ["name"]
            })
        }
    }

    async fn test_req(_input: Valid<Input>) -> String {
        "success".to_owned()
    }

    #[actix_rt::test]
    async fn post_non_json() {
        let _ = env_logger::try_init();

        let mut input = std::collections::HashMap::new();
        input.insert("hello", "world");

        let app = test::init_service(App::new().route("/", web::post().to(test_req))).await;
        let req = test::TestRequest::post()
            .uri("/")
            .append_header(("content-type", "text/plain"))
            .set_form(&input)
            .to_request();
        let response = test::call_service(&app, req).await;

        check!(response.status() == 400);

        check!(response.headers().get("content-type").unwrap() == "application/problem+json");

        let body = actix_web::test::read_body(response).await;

        assert_json_snapshot!(serde_json::from_slice::<Value>(&body).unwrap(), @r###"
        {
          "type": "about:blank",
          "title": "Bad Request",
          "status": 400
        }
        "###);
    }

    #[actix_rt::test]
    async fn post_missing_field() {
        let _ = env_logger::try_init();

        let app = test::init_service(App::new().route("/", web::post().to(test_req))).await;
        let req = test::TestRequest::post().uri("/").set_json(&json!({})).to_request();
        let response = test::call_service(&app, req).await;

        check!(response.status() == 422);

        check!(response.headers().get("content-type").unwrap() == "application/problem+json");

        let body = actix_web::test::read_body(response).await;

        assert_json_snapshot!(serde_json::from_slice::<Value>(&body).unwrap(), @r###"
        {
          "type": "tag:worlds,2021:problems/validation",
          "title": "Request body failed validation",
          "status": 422,
          "validationErrors": [
            {
              "code": "required",
              "title": "This property is required",
              "path": "/name"
            }
          ]
        }
        "###);
    }

    #[actix_rt::test]
    async fn post_long_field() {
        let _ = env_logger::try_init();

        let app = test::init_service(App::new().route("/", web::post().to(test_req))).await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&json!({
              "name": "Too Long"
            }))
            .to_request();
        let response = test::call_service(&app, req).await;

        check!(response.status() == 422);

        check!(response.headers().get("content-type").unwrap() == "application/problem+json");

        let body = actix_web::test::read_body(response).await;

        assert_json_snapshot!(serde_json::from_slice::<Value>(&body).unwrap(), @r###"
        {
          "type": "tag:worlds,2021:problems/validation",
          "title": "Request body failed validation",
          "status": 422,
          "validationErrors": [
            {
              "code": "max_length",
              "title": "MaxLength condition is not met",
              "path": "/name"
            }
          ]
        }
        "###);
    }

    #[actix_rt::test]
    async fn post_wrong_type() {
        let _ = env_logger::try_init();

        let app = test::init_service(App::new().route("/", web::post().to(test_req))).await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&json!({
              "name": 5
            }))
            .to_request();
        let response = test::call_service(&app, req).await;

        check!(response.status() == 422);

        check!(response.headers().get("content-type").unwrap() == "application/problem+json");

        let body = actix_web::test::read_body(response).await;

        assert_json_snapshot!(serde_json::from_slice::<Value>(&body).unwrap(), @r###"
        {
          "type": "tag:worlds,2021:problems/validation",
          "title": "Request body failed validation",
          "status": 422,
          "validationErrors": [
            {
              "code": "wrong_type",
              "title": "Type of the value is wrong",
              "path": "/name",
              "detail": "The value must be string"
            }
          ]
        }
        "###);
    }

    #[actix_rt::test]
    async fn post_valid() {
        let _ = env_logger::try_init();

        let app = test::init_service(App::new().route("/", web::post().to(test_req))).await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&json!({
              "name": "Fine"
            }))
            .to_request();
        let response = test::call_service(&app, req).await;

        check!(response.status() == 200);
    }

    #[actix_rt::test]
    async fn post_extra_fields() {
        let _ = env_logger::try_init();

        let app = test::init_service(App::new().route("/", web::post().to(test_req))).await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&json!({
              "name": "Fine",
              "age": 42
            }))
            .to_request();
        let response = test::call_service(&app, req).await;

        check!(response.status() == 200);
    }
}
