use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;
use serde_json::json;

use crate::tests::{database::seed::SeedUser, suite::TestSuite};

#[actix_rt::test]
async fn empty_body() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/authenticate")
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
async fn blank_fields() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/authenticate")
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

#[actix_rt::test]
async fn unknown_user() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/authenticate")
                .set_json(&json!({
                  "username": "unknown",
                  "password": "password"
                }))
                .to_request(),
        )
        .await;

    check!(response.status == 401);

    check!(response.headers.get("content-type").unwrap() == "application/problem+json");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "about:blank",
      "title": "Unauthorized",
      "status": 401
    }
    "###);
}

#[actix_rt::test]
async fn wrong_password() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        ..SeedUser::default()
    }
    .with_password("password");

    let suite = TestSuite::new().await;
    suite.seed(&user).await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/authenticate")
                .set_json(&json!({
                  "username": "testuser",
                  "password": "wrong"
                }))
                .to_request(),
        )
        .await;

    check!(response.status == 401);

    check!(response.headers.get("content-type").unwrap() == "application/problem+json");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "about:blank",
      "title": "Unauthorized",
      "status": 401
    }
    "###);
}

#[actix_rt::test]
async fn correct_password() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        ..SeedUser::default()
    }
    .with_password("password");

    let suite = TestSuite::new().await;
    suite.seed(&user).await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/authenticate")
                .set_json(&json!({
                  "username": "testuser",
                  "password": "password"
                }))
                .to_request(),
        )
        .await;

    check!(response.status == 200);

    check!(response.headers.get("content-type").unwrap() == "application/json");

    assert_json_snapshot!(response.to_json().unwrap(), {
      ".token" => "[token]",
      ".user_id" => "[user_id]",
      ".expires_at" => "[expires_at]",
    }, @r###"
    {
      "token": "[token]",
      "user_id": "[user_id]",
      "expires_at": "[expires_at]"
    }
    "###);
}
