use super::suite::TestSuite;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;
use serde_json::json;

#[actix_rt::test]
async fn authenticate_empty_body() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate")
                .set_json(&json!({}))
                .to_request(),
        )
        .await;

    check!(response.status == 422);

    check!(response.headers.get("content-type").unwrap() == "application/problem+json");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "about:blank",
      "title": "Unprocessable Entity",
      "status": 422,
      "validationErrors": [
        {
          "code": "required",
          "title": "This property is required",
          "path": "/username"
        },
        {
          "code": "required",
          "title": "This property is required",
          "path": "/password"
        }
      ]
    }
    "###);
}

#[actix_rt::test]
async fn authenticate_blank_fields() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate")
                .set_json(&json!({
                  "username": "",
                  "password": ""
                }))
                .to_request(),
        )
        .await;

    check!(response.status == 422);

    check!(response.headers.get("content-type").unwrap() == "application/problem+json");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "about:blank",
      "title": "Unprocessable Entity",
      "status": 422,
      "validationErrors": [
        {
          "code": "min_length",
          "title": "MinLength condition is not met",
          "path": "/username"
        },
        {
          "code": "min_length",
          "title": "MinLength condition is not met",
          "path": "/password"
        }
      ]
    }
    "###);
}
