use std::fmt::Debug;

use hyper::client::HttpConnector;
use hyper::service::Service;
use hyper::Client as HyperClient;
use hyper::{Body, Request, Response};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

use crate::auth::Authorization;

#[derive(Debug, Clone)]
pub struct Client {
    pub base_path: String,
    pub authorization: Authorization,
    pub inner: HyperClient<HttpsConnector<HttpConnector>, Body>,
}

impl Default for Client {
    fn default() -> Self {
        use hyper_rustls::ConfigBuilderExt;

        let tls = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_native_roots()
            .with_no_client_auth();
        let https = HttpsConnectorBuilder::new()
            .with_tls_config(tls)
            .https_or_http()
            .enable_http1()
            .build();
        let inner = hyper::Client::builder().build(https);
        Self {
            base_path: "https://q.trap.jp/api/v3".to_string(),
            authorization: Authorization::None,
            inner,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Builder {
    pub base_path: Option<String>,
    pub authorization: Authorization,
}

impl Client {
    pub fn builder() -> Builder {
        Default::default()
    }
}

impl Builder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn base_path(self, base_path: &str) -> Self {
        Self {
            base_path: Some(base_path.to_string()),
            ..self
        }
    }

    pub fn authorization_bearer(self, bearer: &str) -> Self {
        let authorization = Authorization::Bearer(bearer.to_string());
        Self {
            authorization,
            ..self
        }
    }

    pub fn authorization(self, authorization: Authorization) -> Self {
        Self {
            authorization,
            ..self
        }
    }

    pub fn build(self) -> Client {
        let Builder {
            base_path,
            authorization,
        } = self;
        let client: Client = Default::default();

        let client = if let Some(base_path) = base_path {
            Client {
                base_path,
                ..client
            }
        } else {
            client
        };

        if let Authorization::Bearer(bearer) = authorization {
            Client {
                authorization: Authorization::Bearer(bearer),
                ..client
            }
        } else {
            client
        }
    }
}

impl Service<Request<Body>> for Client {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = hyper::client::ResponseFuture;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        if let Authorization::Bearer(bearer) = &self.authorization {
            let headers = req.headers_mut();
            headers.insert(
                hyper::header::AUTHORIZATION,
                format!("Bearer {}", bearer).parse().unwrap(),
            );
        }
        self.inner.call(req)
    }
}
