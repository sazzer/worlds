use crate::tests::{database::seed::SeedUser, suite::TestSuite};
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;
use serde_json::json;

#[actix_rt::test]
async fn empty_body() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/check")
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
                .uri("/authenticate/check")
                .set_json(&json!({
                  "username": ""
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
        }
      ]
    }
    "###);
}

#[actix_rt::test]
async fn unknown_user() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/check")
                .set_json(&json!({
                  "username": "unknown"
                }))
                .to_request(),
        )
        .await;

    check!(response.status == 200);

    check!(response.headers.get("content-type").unwrap() == "application/json");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "known": false
    }
    "###);
}

#[actix_rt::test]
async fn known_user() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        ..SeedUser::default()
    };

    let suite = TestSuite::new().await;
    suite.seed(&user).await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/check")
                .set_json(&json!({
                  "username": "testuser"
                }))
                .to_request(),
        )
        .await;

    check!(response.status == 200);

    check!(response.headers.get("content-type").unwrap() == "application/json");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "known": true
    }
    "###);
}
