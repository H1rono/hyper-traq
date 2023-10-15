use hyper::body::Bytes;
use hyper::{Body, Method};

use super::ApiRequest;

#[derive(Debug, Clone, Copy)]
pub struct GetUsers;

#[derive(Debug, Clone)]
pub struct Users(String);

impl ApiRequest for GetUsers {
    // TODO: parse response with serde_json
    type Response = String;
    type Error = std::str::Utf8Error;

    fn uri(&self) -> &str {
        "/users"
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Body {
        Body::empty()
    }

    fn parse(&self, body: Bytes) -> Result<Self::Response, Self::Error> {
        let s = std::str::from_utf8(&body)?;
        Ok(s.to_string())
    }
}
