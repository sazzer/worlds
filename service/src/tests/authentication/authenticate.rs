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
        user_id: "4ea96dc3-df11-43c0-8a33-a0813f03937f".parse().unwrap(),
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
      ".expires_at" => "[expires_at]",
    }, @r###"
    {
      "token": "[token]",
      "user_id": "4ea96dc3-df11-43c0-8a33-a0813f03937f",
      "expires_at": "[expires_at]"
    }
    "###);
}

#[actix_rt::test]
async fn use_token() {
    let user = SeedUser {
        user_id: "4ea96dc3-df11-43c0-8a33-a0813f03937f".parse().unwrap(),
        version: "d61dac0c-45f2-49ed-85cc-f24bbe939404".parse().unwrap(),
        username: "testuser".to_owned(),
        email: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
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

    let response = response.to_json().unwrap();
    let token = response.get("token").unwrap().as_str().unwrap();

    let response = suite
        .inject(
            TestRequest::get()
                .uri("/users/4ea96dc3-df11-43c0-8a33-a0813f03937f")
                .append_header(("Authorization", format!("Bearer {}", token)))
                .to_request(),
        )
        .await;

    check!(response.status == 200);

    check!(response.headers.get("content-type").unwrap() == "application/json");
    check!(response.headers.get("cache-control").unwrap() == "private, max-age=3600");
    check!(response.headers.get("etag").unwrap() == "\"d61dac0c-45f2-49ed-85cc-f24bbe939404\"");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "userId": "4ea96dc3-df11-43c0-8a33-a0813f03937f",
      "username": "testuser",
      "email": "testuser@example.com",
      "displayName": "Test User"
    }
    "###);
}
