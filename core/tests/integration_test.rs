use core::model::*;
use serde_json::json;

#[test]
fn test_full_request_lifecycle() {
    let request = HttpRequest {
        method: HttpMethod::POST,
        url: url::Url::parse("https://httpbin.org/post").unwrap(),
        headers: std::collections::HashMap::new(),
        body: Some(RequestBody::Json(json!({"test": true}))),
    };

    let json_str = serde_json::to_string(&request).unwrap();
    let deserialized: HttpRequest = serde_json::from_str(&json_str).unwrap();

    assert_eq!(request.method, deserialized.method);
}
