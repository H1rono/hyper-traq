use std::error::Error as StdError;
use std::fmt::Debug;
use std::future::Future;
use std::io::Error as IoError;
use std::pin::Pin;
use std::str::Utf8Error;

use hyper::body::Bytes;
use hyper::http::StatusCode;
use hyper::service::Service;
use hyper::{Body, Method, Request};
use image::ImageError;
use thiserror::Error as ThisError;

use crate::auth::Authorization;
use crate::client::Client;

pub mod me;
pub mod users;

pub trait ApiRequest: Sync + Send + 'static {
    type Response: Sync + Send + 'static;
    type Error: StdError + Sync + Send + 'static;

    fn uri(&self) -> String;
    fn method(&self) -> Method;
    fn content_type(&self) -> Option<String> {
        None
    }
    fn body(&self) -> Body;
    fn parse(&self, body: Bytes) -> Result<Self::Response, Self::Error>;
}

#[derive(Debug, ThisError)]
pub enum Error<T: StdError + Debug + Sync + Send + 'static> {
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    HyperHttp(#[from] hyper::http::Error),
    #[error("bad status code: {0}")]
    BadStatus(StatusCode, Bytes),
    #[error(transparent)]
    Custom(T),
}

impl Client {
    pub async fn request<Req>(&self, req: Req) -> Result<Req::Response, Error<Req::Error>>
    where
        Req: ApiRequest + Clone + Sync + Send + 'static,
    {
        use hyper::header::AUTHORIZATION;
        let uri = format!("{}{}", &self.base_path, req.uri());
        let req_builder = Request::builder().method(req.method()).uri(uri);
        let req_builder = if let Authorization::Bearer(bearer) = &self.authorization {
            req_builder.header(AUTHORIZATION, format!("Bearer {}", bearer))
        } else {
            req_builder
        };
        let req_builder = if let Some(content_type) = req.content_type() {
            use hyper::header::CONTENT_TYPE;
            req_builder.header(CONTENT_TYPE, content_type)
        } else {
            req_builder
        };
        let raw_req = req_builder.body(req.body()).map_err(Error::HyperHttp)?;
        let res = self.inner.request(raw_req).await.map_err(Error::Hyper)?;
        let (parts, body) = res.into_parts();
        let status = parts.status;
        let bytes = hyper::body::to_bytes(body).await.map_err(Error::Hyper)?;
        if !status.is_success() {
            Err(Error::BadStatus(status, bytes))
        } else {
            req.parse(bytes).map_err(Error::Custom)
        }
    }
}

impl<Req> Service<Req> for Client
where
    Req: ApiRequest + Clone + Sync + Send + 'static,
{
    type Response = Req::Response;
    type Error = Error<Req::Error>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Error::Hyper)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let s = self.clone();
        Box::pin(async move { s.request(req).await })
    }
}

#[derive(Debug, ThisError)]
pub enum ApiError {
    #[error(transparent)]
    Io(#[from] IoError),
    #[error(transparent)]
    Utf8(#[from] Utf8Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Image(#[from] ImageError),
}
