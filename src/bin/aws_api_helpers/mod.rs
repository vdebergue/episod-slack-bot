use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use aws_lambda;
use http;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ShortApiGatewayProxyRequest {
    pub path: String,
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    #[serde(default, deserialize_with = "nullable_default")]
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    #[serde(
        rename = "queryStringParameters",
        deserialize_with = "nullable_default"
    )]
    pub query_string_parameters: HashMap<String, String>,
}

fn nullable_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_else(T::default))
}

pub struct Response(pub aws_lambda::event::apigw::ApiGatewayProxyResponse);

impl<'a> From<http::response::Response<&'a str>> for Response {
    fn from(response: http::response::Response<&'a str>) -> Self {
        Response(aws_lambda::event::apigw::ApiGatewayProxyResponse {
            body: Some(response.body().to_string()),
            status_code: i64::from(response.status().as_u16()),
            is_base64_encoded: None,
            headers: HashMap::new(),
        })
    }
}

impl From<http::response::Response<String>> for Response {
    fn from(response: http::response::Response<String>) -> Self {
        Response(aws_lambda::event::apigw::ApiGatewayProxyResponse {
            body: Some(response.body().to_string()),
            status_code: i64::from(response.status().as_u16()),
            is_base64_encoded: None,
            headers: HashMap::new(),
        })
    }
}

#[test]
fn can_convert_from_response() {
    let response: Response = http::response::Builder::new()
        .status(200)
        .header("my-header", "my-header-value")
        .body("my body")
        .expect("could not build response")
        .into();

    assert_eq!(
        response.0,
        aws_lambda::event::apigw::ApiGatewayProxyResponse {
            body: Some("my body".to_string()),
            status_code: 200,
            is_base64_encoded: None,
            headers: HashMap::new(),
        }
    );
}

#[derive(Debug, Fail)]
pub enum HttpError {
    #[fail(display = "unexpected path: {} {}", method, path)]
    UnexpectedPath { method: String, path: String },
}
