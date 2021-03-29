use super::service::TestService;
use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
pub async fn test_home_document() {
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
