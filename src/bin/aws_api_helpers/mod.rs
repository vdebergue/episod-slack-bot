use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde_json;
use serde_urlencoded;

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

impl<'a, T> Into<http::request::Request<T>> for &'a ShortApiGatewayProxyRequest
where
    T: Default,
    T: Deserialize<'a>,
{
    fn into(self) -> http::request::Request<T> {
        let method: &str = self.http_method.as_ref();
        let path: &str = self.path.as_ref();

        let mut builder = http::request::Request::builder();
        builder.method(method);
        if self.query_string_parameters.is_empty() {
            builder.uri(path);
        } else {
            builder.uri(format!(
                "{}?{}",
                path,
                serde_urlencoded::to_string(&self.query_string_parameters)
                    .expect("couldn't read query parameters")
            ));
        };

        self.headers.iter().for_each(|(key, value)| {
            let key: &str = key.as_ref();
            let value: &str = value.as_ref();
            builder.header(key, value);
        });

        if let Some(ref body) = self.body {
            builder.body(serde_json::from_str::<T>(body.as_ref()).expect(""))
        } else {
            builder.body(T::default())
        }.expect("Couldn't build request")
    }
}

#[test]
fn can_convert_to_request() {
    extern crate serde_json;

    let api_gateway_proxy_event = r#"
{
  "body": "{\"test\":\"body\"}",
  "resource": "/{proxy+}",
  "requestContext": {
    "resourceId": "123456",
    "apiId": "1234567890",
    "resourcePath": "/{proxy+}",
    "httpMethod": "POST",
    "requestId": "c6af9ac6-7b61-11e6-9a41-93e8deadbeef",
    "accountId": "123456789012",
    "identity": {
      "apiKey": null,
      "userArn": null,
      "cognitoAuthenticationType": null,
      "caller": null,
      "userAgent": "Custom User Agent String",
      "user": null,
      "cognitoIdentityPoolId": null,
      "cognitoIdentityId": null,
      "cognitoAuthenticationProvider": null,
      "sourceIp": "127.0.0.1",
      "accountId": null
    },
    "stage": "prod"
  },
  "queryStringParameters": {
    "foo": "bar"
  },
  "headers": {
    "Via": "1.1 08f323deadbeefa7af34d5feb414ce27.cloudfront.net (CloudFront)",
    "Accept-Language": "en-US,en;q=0.8",
    "CloudFront-Is-Desktop-Viewer": "true",
    "CloudFront-Is-SmartTV-Viewer": "false",
    "CloudFront-Is-Mobile-Viewer": "false",
    "X-Forwarded-For": "127.0.0.1, 127.0.0.2",
    "CloudFront-Viewer-Country": "US",
    "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
    "Upgrade-Insecure-Requests": "1",
    "X-Forwarded-Port": "443",
    "Host": "1234567890.execute-api.us-east-1.amazonaws.com",
    "X-Forwarded-Proto": "https",
    "X-Amz-Cf-Id": "cDehVQoZnx43VYQb9j2-nvCh-9z396Uhbp027Y2JvkCPNLmGJHqlaA==",
    "CloudFront-Is-Tablet-Viewer": "false",
    "Cache-Control": "max-age=0",
    "User-Agent": "Custom User Agent String",
    "CloudFront-Forwarded-Proto": "https",
    "Accept-Encoding": "gzip, deflate, sdch"
  },
  "pathParameters": {
    "proxy": "path/to/resource"
  },
  "httpMethod": "POST",
  "stageVariables": {
    "baz": "qux"
  },
  "path": "/path/to/resource"
}
"#;

    let short = serde_json::from_str::<ShortApiGatewayProxyRequest>(api_gateway_proxy_event);
    assert!(short.is_ok());
    let short = &short.unwrap();

    let request: http::Request<HashMap<String, String>> = short.into();
    assert_eq!(request.method(), "POST");
    assert_eq!(request.uri(), "/path/to/resource?foo=bar");
    let mut body = HashMap::new();
    body.insert("test".to_string(), "body".to_string());
    assert_eq!(*request.body(), body);
    assert_eq!(
        request
            .headers()
            .get("x-forwarded-proto")
            .map(|v| v.to_str().unwrap()),
        Some("https"),
    );
    assert_eq!(
        request.headers().get("via").map(|v| v.to_str().unwrap()),
        Some("1.1 08f323deadbeefa7af34d5feb414ce27.cloudfront.net (CloudFront)"),
    );
    assert_eq!(
        request
            .headers()
            .get("x-amz-cf-id")
            .map(|v| v.to_str().unwrap()),
        Some("cDehVQoZnx43VYQb9j2-nvCh-9z396Uhbp027Y2JvkCPNLmGJHqlaA=="),
    );
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
