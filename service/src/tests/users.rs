use super::suite::TestSuite;
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
