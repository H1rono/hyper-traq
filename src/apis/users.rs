use std::str::Utf8Error;

use hyper::body::Bytes;
use hyper::{Body, Method};
use thiserror::Error as ThisError;
use uuid::Uuid;

use super::ApiRequest;
use crate::models::{PatchUserRequest, User, UserTags, Users};

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Utf8(#[from] Utf8Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

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
    type Response = Users;
    type Error = Error;

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
        let r = serde_json::from_str(s)?;
        Ok(r)
    }
}

#[derive(Debug, Clone)]
pub struct GetUser {
    id: Uuid,
}

impl GetUser {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

impl ApiRequest for GetUser {
    type Response = User;
    type Error = Error;

    fn uri(&self) -> String {
        format!("/users/{}", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Body {
        Body::empty()
    }

    fn parse(&self, body: Bytes) -> Result<Self::Response, Self::Error> {
        let s = std::str::from_utf8(&body)?;
        let r = serde_json::from_str(s)?;
        Ok(r)
    }
}

#[derive(Debug, Clone)]
pub struct GetUserTags {
    id: Uuid,
}

impl GetUserTags {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

impl ApiRequest for GetUserTags {
    type Response = UserTags;
    type Error = Error;

    fn uri(&self) -> String {
        format!("/users/{}/tags", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Body {
        Body::empty()
    }

    fn parse(&self, body: Bytes) -> Result<Self::Response, Self::Error> {
        let s = std::str::from_utf8(&body)?;
        let r = serde_json::from_str(s)?;
        Ok(r)
    }
}

/// maybe works
#[derive(Debug, Clone)]
pub struct PatchUser {
    id: Uuid,
    request: PatchUserRequest,
}

impl PatchUser {
    pub fn new(id: Uuid, request: PatchUserRequest) -> Self {
        Self { id, request }
    }
}

impl ApiRequest for PatchUser {
    type Response = ();
    type Error = std::convert::Infallible;

    fn uri(&self) -> String {
        format!("/users/{}", self.id)
    }

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn content_type(&self) -> Option<String> {
        Some("application/json".to_string())
    }

    fn body(&self) -> Body {
        serde_json::to_string(&self.request)
            .expect("failed to serialize PatchUserRequest")
            .into()
    }

    fn parse(&self, _body: Bytes) -> Result<Self::Response, Self::Error> {
        Ok(())
    }
}
