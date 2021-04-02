use super::service::TestService;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
pub async fn unauthorized() {
    let test_service = TestService::new().await;

    let response = test_service.inject(TestRequest::get().uri("/").to_request()).await;

    check!(response.status == 200);

    check!(response.headers.get("content-type").unwrap() == "application/hal+json");
    check!(response.headers.get("cache-control").unwrap() == "public, max-age=3600");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "name": "worlds_service",
      "version": "0.1.0",
      "_links": {
        "self": {
          "href": "/"
        }
      }
    }
    "###);
}

#[actix_rt::test]
pub async fn invalid_authorization() {
    let test_service = TestService::new().await;

    let response = test_service
        .inject(TestRequest::get().uri("/").header("authorization", "Basic abc").to_request())
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
pub async fn authorized() {
    let test_service = TestService::new().await;

    let response = test_service
        .inject(TestRequest::get().uri("/").set(test_service.authorization("myUserId")).to_request())
        .await;

    check!(response.status == 200);

    check!(response.headers.get("content-type").unwrap() == "application/hal+json");
    check!(response.headers.get("cache-control").unwrap() == "public, max-age=3600");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "name": "worlds_service",
      "version": "0.1.0",
      "_links": {
        "self": {
          "href": "/"
        },
        "tag:worlds,2021:rels/user": {
          "href": "/users/myUserId"
        }
      }
    }
    "###);
}
