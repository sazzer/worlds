use super::{database::seed::SeedUser, suite::TestSuite};
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn get_unknown_user() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::get()
                .uri("/users/4ea96dc3-df11-43c0-8a33-a0813f03937f")
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

#[actix_rt::test]
async fn get_invalid_user_id() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(TestRequest::get().uri("/users/invalid").to_request())
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

#[actix_rt::test]
async fn get_valid_user_id() {
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
            TestRequest::get()
                .uri("/users/4ea96dc3-df11-43c0-8a33-a0813f03937f")
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

#[actix_rt::test]
async fn get_invalid_authorization() {
    let suite = TestSuite::new().await;

    let response = suite
        .inject(
            TestRequest::get()
                .uri("/users/4ea96dc3-df11-43c0-8a33-a0813f03937f")
                .append_header(("Authorization", "Invalid"))
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
