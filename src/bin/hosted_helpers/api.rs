use std::net::TcpListener;
use std::os::unix::io::FromRawFd;
use std::str;
use std::time::Duration;

use uuid;

use actix_web::middleware::cors;
use actix_web::{
    client, http, http::StatusCode, middleware, server, App, HttpMessage, HttpRequest,
    HttpResponse, Result,
};
use failure;
use futures::future::Future;

use episod::episod;
use hosted_helpers::api_helpers;

fn index(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../../static/welcome.html")))
}

pub fn get_planning(
    filters: api_helpers::Query<episod::Filters>,
) -> impl Future<Item = HttpResponse, Error = failure::Error> {
    client::get("https://www.episod.com/planning/")
        .timeout(Duration::new(30, 0))
        .finish()
        .unwrap()
        .send()
        .map_err(|err| err.into())
        .and_then(|resp| resp.body().limit(1_048_576).map_err(|err| err.into()))
        .map(move |body| {
            episod::extract_sessions_and_filter(str::from_utf8(&body.to_vec()).unwrap(), &filters)
        }).map(|sessions| HttpResponse::Ok().json(sessions))
}

fn http_application() -> App {
    App::new()
        .middleware(middleware::DefaultHeaders::new().header(
            "X-Request-Id",
            uuid::Uuid::new_v4().to_hyphenated().to_string().as_str(),
        )).middleware(middleware::Logger::new(
            "%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %{X-Request-Id}o - %T",
        )).middleware(cors::Cors::build().send_wildcard().finish())
        .resource("/", |r| r.method(http::Method::GET).f(index))
        .resource("/planning", |r| {
            r.method(http::Method::GET).with_async(get_planning)
        }).default_resource(|r| {
            r.f(api_helpers::p404);
        })
}

pub fn serve(host: &str, port: u16) {
    server::new(http_application)
        .bind(format!("{}:{}", host, port))
        .unwrap()
        .run();
}

pub fn serve_from_fd(fd: &str) {
    server::new(http_application)
        .listen(unsafe { TcpListener::from_raw_fd(fd.parse().unwrap()) })
        .run();
}
