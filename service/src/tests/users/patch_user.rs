use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;
use serde_json::json;

use crate::tests::{database::seed::SeedUser, suite::TestSuite};

#[actix_rt::test]
async fn invalid_user_id() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(TestRequest::patch().uri("/users/invalid").set_json(&json!({})).to_request())
        .await;

    check!(response.status == 403);

    check!(response.headers.get("content-type").unwrap() == "application/problem+json");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "about:blank",
      "title": "Forbidden",
      "status": 403
    }
    "###);
}

#[actix_rt::test]
async fn valid_user_id_unauthenticated() {
    let user = SeedUser {
        user_id: "4ea96dc3-df11-43c0-8a33-a0813f03937f".parse().unwrap(),
        version: "d61dac0c-45f2-49ed-85cc-f24bbe939404".parse().unwrap(),
        username: "testuser".to_owned(),
        email: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };

    let suite = TestSuite::new().await;
    suite.seed(&user).await;

    let response = suite
        .inject(
            TestRequest::patch()
                .uri("/users/4ea96dc3-df11-43c0-8a33-a0813f03937f")
                .set_json(&json!({}))
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
async fn invalid_authorization() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::patch()
                .uri("/users/4ea96dc3-df11-43c0-8a33-a0813f03937f")
                .append_header(("Authorization", "Invalid"))
                .set_json(&json!({}))
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
async fn valid_user_id_wrong_user() {
    let user = SeedUser {
        user_id: "4ea96dc3-df11-43c0-8a33-a0813f03937f".parse().unwrap(),
        version: "d61dac0c-45f2-49ed-85cc-f24bbe939404".parse().unwrap(),
        username: "testuser".to_owned(),
        email: "testuser@example.com".to_owned(),
        display_name: "Test User".to_owned(),
        ..SeedUser::default()
    };

    let suite = TestSuite::new().await;
    suite.seed(&user).await;

    let response = suite
        .inject(
            TestRequest::patch()
                .uri("/users/4ea96dc3-df11-43c0-8a33-a0813f03937f")
                .append_header(suite.authenticate("37f35c28-1c26-465d-9a45-b87e59a9760a"))
                .set_json(&json!({}))
                .to_request(),
        )
        .await;

    check!(response.status == 403);

    check!(response.headers.get("content-type").unwrap() == "application/problem+json");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
        {
          "type": "about:blank",
          "title": "Forbidden",
          "status": 403
        }
        "###);
}

#[actix_rt::test]
async fn unknown_user() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::patch()
                .uri("/users/4ea96dc3-df11-43c0-8a33-a0813f03937f")
                .append_header(suite.authenticate("4ea96dc3-df11-43c0-8a33-a0813f03937f"))
                .set_json(&json!({}))
                .to_request(),
        )
        .await;

    check!(response.status == 404);

    check!(response.headers.get("content-type").unwrap() == "application/problem+json");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
        {
          "type": "about:blank",
          "title": "The requested resource was not found",
          "status": 404
        }
        "###);
}
