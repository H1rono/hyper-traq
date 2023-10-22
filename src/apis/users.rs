use std::str::Utf8Error;

use hyper::body::Bytes;
use hyper::{Body, Method};
use itertools::Itertools;
use thiserror::Error as ThisError;
use uuid::Uuid;

use super::ApiRequest;
use crate::models::{
    Message, Messages, PatchUserRequest, PostMessageRequest, User, UserStats, UserTags, Users,
};

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

#[derive(Debug, Clone)]
pub struct PostDirectMessage {
    id: Uuid,
    request: PostMessageRequest,
}

impl PostDirectMessage {
    pub fn new(id: Uuid, request: PostMessageRequest) -> Self {
        Self { id, request }
    }
}

impl ApiRequest for PostDirectMessage {
    type Response = Message;
    type Error = Error;

    fn uri(&self) -> String {
        format!("/users/{}/messages", self.id)
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn content_type(&self) -> Option<String> {
        Some("application/json".to_string())
    }

    fn body(&self) -> Body {
        serde_json::to_string(&self.request)
            .expect("failed to serialize PostMessageRequest")
            .into()
    }

    fn parse(&self, body: Bytes) -> Result<Self::Response, Self::Error> {
        let s = std::str::from_utf8(&body)?;
        let r = serde_json::from_str(s)?;
        Ok(r)
    }
}

#[derive(Debug, Clone)]
pub struct GetDirectMessages {
    id: Uuid,
    limit: Option<u32>,
    offset: Option<u32>,
    since: Option<String>,
    until: Option<String>,
    inclusive: Option<bool>,
    order: Option<String>,
}

impl GetDirectMessages {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            limit: None,
            offset: None,
            since: None,
            until: None,
            inclusive: None,
            order: None,
        }
    }

    pub fn set_limit(self, limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }

    pub fn set_offset(self, offset: u32) -> Self {
        Self {
            offset: Some(offset),
            ..self
        }
    }

    pub fn set_since(self, since: &str) -> Self {
        Self {
            since: Some(since.to_string()),
            ..self
        }
    }

    pub fn set_until(self, until: &str) -> Self {
        Self {
            until: Some(until.to_string()),
            ..self
        }
    }

    pub fn set_inclusive(self, inclusive: bool) -> Self {
        Self {
            inclusive: Some(inclusive),
            ..self
        }
    }

    pub fn set_order(self, order: &str) -> Self {
        Self {
            order: Some(order.to_string()),
            ..self
        }
    }
}

impl ApiRequest for GetDirectMessages {
    type Response = Messages;
    type Error = Error;

    fn uri(&self) -> String {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(limit) = self.limit {
            query.push(("limit", limit.to_string()));
        }
        if let Some(offset) = self.offset {
            query.push(("offset", offset.to_string()));
        }
        if let Some(since) = &self.since {
            query.push(("since", since.clone()));
        }
        if let Some(until) = &self.until {
            query.push(("until", until.clone()))
        }
        if let Some(inclusive) = self.inclusive {
            query.push(("inclusive", inclusive.to_string()));
        }
        if let Some(order) = &self.order {
            query.push(("order", order.clone()));
        }
        let s = format!("/users/{}/messages", self.id);
        if query.is_empty() {
            return s;
        }
        let query = query
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .join("&");
        format!("{}?{}", s, query)
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
pub struct GetUserStats {
    id: Uuid,
}

impl GetUserStats {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

impl ApiRequest for GetUserStats {
    type Response = UserStats;
    type Error = Error;

    fn uri(&self) -> String {
        format!("/users/{}/stats", self.id)
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
