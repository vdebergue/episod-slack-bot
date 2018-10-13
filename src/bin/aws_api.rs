extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;

extern crate http;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "aws")]
extern crate aws_lambda;
#[cfg(feature = "aws")]
extern crate reqwest;
#[cfg(feature = "aws")]
extern crate rusoto_core;
#[cfg(feature = "aws")]
extern crate rusoto_sns;

extern crate episod;

use std::collections::HashMap;

use rusoto_sns::Sns;

lazy_static! {
    pub static ref PLANNING_HTML: String = {
        reqwest::get("https://www.episod.com/planning/")
            .unwrap()
            .text()
            .unwrap()
    };
}

mod aws_api_helpers;

pub fn hello(
    _req: &http::request::Request<&str>,
) -> Result<http::response::Response<String>, failure::Error> {
    Ok(http::response::Builder::new()
        .status(200)
        .body(include_str!("../../static/welcome.html").to_string())?)
}

pub fn planning(
    req: &aws_api_helpers::ShortApiGatewayProxyRequest,
) -> Result<http::response::Response<String>, failure::Error> {
    let body = serde_json::to_string(&episod::episod::extract_sessions_and_filter(
        &reqwest::get("https://www.episod.com/planning/")
            .unwrap()
            .text()
            .unwrap(),
        &serde_urlencoded::from_str(
            &req.query_string_parameters
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("&"),
        ).unwrap(),
    )).unwrap();
    Ok(http::response::Builder::new().status(200).body(body)?)
}

use std::env;

pub fn slack_event(
    req: &aws_api_helpers::ShortApiGatewayProxyRequest,
) -> Result<http::response::Response<String>, failure::Error> {
    let event: episod::slack::Event = serde_json::from_str(&req.clone().body.unwrap())?;
    match event {
        episod::slack::Event::UrlVerification { challenge, .. } => {
            Ok(http::response::Builder::new().status(200).body(challenge)?)
        }
        episod::slack::Event::EventCallback { event, token, .. } => match event {
            episod::slack::EventCallback::AppMention { channel, text, .. } => {
                let client = rusoto_sns::SnsClient::new(rusoto_core::Region::UsEast1);
                client
                    .publish(rusoto_sns::PublishInput {
                        message: serde_json::to_string(&Notification {
                            token,
                            channel,
                            query: text,
                        }).unwrap(),
                        topic_arn: Some(env::var("topic").unwrap()),
                        ..Default::default()
                    }).sync()
                    .unwrap();
                Ok(http::response::Builder::new()
                    .status(200)
                    .body("".to_string())?)
            }
        },
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Notification {
    token: String,
    channel: String,
    query: String,
}

fn main() {
    aws_lambda::start(|req: aws_api_helpers::ShortApiGatewayProxyRequest| {
        let response = match (req.http_method.as_ref(), req.path.as_ref()) {
            ("GET", "/welcome") => hello(&(&req).into()),
            ("GET", "/planning") => planning(&req),
            ("POST", "/slack-event") => slack_event(&req),
            (method, path) => Err(aws_api_helpers::HttpError::UnexpectedPath {
                method: method.to_string(),
                path: path.to_string(),
            }.into()),
        };

        Ok(match response {
            Ok(response) => {
                let a: aws_api_helpers::Response = response.into();
                a.0
            }
            Err(err) => aws_lambda::event::apigw::ApiGatewayProxyResponse {
                body: Some(format!("{}", err)),
                status_code: 500,
                is_base64_encoded: None,
                headers: HashMap::new(),
            },
        })
    })
}
