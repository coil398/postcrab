use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestBody {
    Json(serde_json::Value),
    Text(String),
    Form(HashMap<String, String>),
    Binary(Vec<u8>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: Url,
    pub headers: HashMap<String, String>,
    pub body: Option<RequestBody>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_http_method_serialization() {
        let method = HttpMethod::POST;
        let json_str = serde_json::to_string(&method).unwrap();
        assert_eq!(json_str, "\"POST\"");
    }

    #[test]
    fn test_request_body_json() {
        let body = RequestBody::Json(json!({
            "username": "alice",
            "password": "secret"
        }));

        let serialized = serde_json::to_string(&body).unwrap();
        assert!(serialized.contains("username"));
        assert!(serialized.contains("alice"));
    }

    #[test]
    fn test_http_request_creation() {
        let request = HttpRequest {
            method: HttpMethod::GET,
            url: Url::parse("https://api.example.com/users").unwrap(),
            headers: HashMap::new(),
            body: None,
        };

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.url.as_str(), "https://api.example.com/users");
        assert!(request.body.is_none());
    }
}
