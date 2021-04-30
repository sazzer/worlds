use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;
use serde_json::json;

use crate::tests::{database::seed::SeedUser, suite::TestSuite};

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
      "type": "tag:worlds,2021:problems/validation",
      "title": "Request body failed validation",
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
      "type": "tag:worlds,2021:problems/validation",
      "title": "Request body failed validation",
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

#[actix_rt::test]
async fn success() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/register")
                .set_json(&json!({
                  "username": "testuser",
                  "email": "testuser@example.com",
                  "displayName": "Test User",
                  "password": "testuser123"
                }))
                .to_request(),
        )
        .await;

    check!(response.status == 200);

    check!(response.headers.get("content-type").unwrap() == "application/json");

    assert_json_snapshot!(response.to_json().unwrap(), {
        ".user_id" => "[user_id]",
        ".token" => "[token]",
          ".expires_at" => "[expires_at]",
        }, @r###"
        {
          "token": "[token]",
          "user_id": "[user_id]",
          "expires_at": "[expires_at]"
        }
        "###);
}

#[actix_rt::test]
async fn success_refetch() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/register")
                .set_json(&json!({
                  "username": "testuser",
                  "email": "testuser@example.com",
                  "displayName": "Test User",
                  "password": "testuser123"
                }))
                .to_request(),
        )
        .await;

    check!(response.status == 200);

    let response = response.to_json().unwrap();
    let user_id = response.get("user_id").unwrap().as_str().unwrap();
    let token = response.get("token").unwrap().as_str().unwrap();

    let response = suite
        .inject(
            TestRequest::get()
                .uri(&format!("/users/{}", user_id))
                .append_header(("Authorization", format!("Bearer {}", token)))
                .to_request(),
        )
        .await;

    check!(response.status == 200);

    check!(response.headers.get("content-type").unwrap() == "application/json");

    assert_json_snapshot!(response.to_json().unwrap(), {
        ".userId" => "[user_id]"
      }, @r###"
      {
        "userId": "[user_id]",
        "username": "testuser",
        "email": "testuser@example.com",
        "displayName": "Test User"
      }"###);
}

#[actix_rt::test]
async fn duplicate_username() {
    let user = SeedUser {
        username: "testuser".to_owned(),
        ..SeedUser::default()
    };

    let suite = TestSuite::new().await;
    suite.seed(&user).await;

    let response = suite
        .inject(
            TestRequest::post()
                .uri("/authenticate/register")
                .set_json(&json!({
                  "username": "testuser",
                  "email": "testuser@example.com",
                  "displayName": "Test User",
                  "password": "testuser123"
                }))
                .to_request(),
        )
        .await;

    check!(response.status == 422);

    check!(response.headers.get("content-type").unwrap() == "application/problem+json");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:worlds,2021:problems/authentication/register/duplicate_username",
      "title": "Duplicate Username",
      "status": 422
    }
    "###);
}
