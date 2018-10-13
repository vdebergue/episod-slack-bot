use actix_web::http::StatusCode;
use actix_web::{error, HttpRequest, HttpResponse, Result};

#[derive(Serialize)]
pub struct Message {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

pub fn p404(req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND).json(Message {
        message: "not found".to_string(),
        path: Some(req.path().to_string()),
        method: Some(req.method().as_str().to_string()),
    }))
}

use actix_web::error::Error;
use actix_web::FromRequest;
use serde::de;
use serde_urlencoded;
use std::fmt;
use std::ops::{Deref, DerefMut};

pub struct Query<T>(T);

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Query<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, S> FromRequest<S> for Query<T>
where
    T: de::DeserializeOwned,
{
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        serde_urlencoded::from_str::<T>(req.query_string())
            .map_err(|e| {
                error::InternalError::from_response(
                    "missing query parameter",
                    HttpResponse::build(StatusCode::BAD_REQUEST).json(Message {
                        message: format!("{}", e),
                        path: None,
                        method: None,
                    }),
                ).into()
            }).map(Query)
    }
}

impl<T: fmt::Debug> fmt::Debug for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for Query<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
