use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;
use serde_json::json;

use crate::tests::suite::TestSuite;

#[actix_rt::test]
async fn empty_body() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(TestRequest::post().uri("/authenticate/register").set_json(&json!({})).to_request())
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
          "path": "/displayName"
        },
        {
          "code": "required",
          "title": "This property is required",
          "path": "/email"
        },
        {
          "code": "required",
          "title": "This property is required",
          "path": "/password"
        },
        {
          "code": "required",
          "title": "This property is required",
          "path": "/username"
        }
      ]
    }
    "###);
}

#[actix_rt::test]
async fn blank_fields() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/register")
                .set_json(&json!({
                  "username": "",
                  "email": "",
                  "displayName": "",
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
          "path": "/displayName"
        },
        {
          "code": "min_length",
          "title": "MinLength condition is not met",
          "path": "/email"
        },
        {
          "code": "pattern",
          "title": "Pattern condition is not met",
          "path": "/email"
        },
        {
          "code": "min_length",
          "title": "MinLength condition is not met",
          "path": "/password"
        },
        {
          "code": "min_length",
          "title": "MinLength condition is not met",
          "path": "/username"
        }
      ]
    }
    "###);
}
