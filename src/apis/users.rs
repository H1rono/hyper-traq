use std::str::Utf8Error;

use hyper::body::Bytes;
use hyper::{Body, Method};

use super::ApiRequest;

#[derive(Debug, Clone, Copy)]
pub struct GetUsers;

#[derive(Debug, Clone, Default)]
pub struct GetUsers {
    pub include_suspended: bool,
    pub name: Option<String>,
}

impl GetUsers {
    pub fn new(include_suspended: bool, name: Option<String>) -> Self {
        Self {
            include_suspended,
            name,
        }
    }
}

impl ApiRequest for GetUsers {
    // TODO: parse response with serde_json
    type Response = String;
    type Error = Utf8Error;

    fn uri(&self) -> String {
        let uri = format!("/users?include-suspended={}", self.include_suspended);
        if let Some(name) = &self.name {
            format!("{}&name={}", uri, name)
        } else {
            uri
        }
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
