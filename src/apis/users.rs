use std::convert::Infallible;
use std::io::Cursor;

use hyper::body::Bytes;
use hyper::{Body, Method};
use image::ImageFormat;
use itertools::Itertools;
use uuid::Uuid;

use super::{ApiError, ApiRequest};
use crate::models::{
    DirectMessageChannel, Image, Message, Messages, PatchUserRequest, PatchUserTagRequest,
    PostMessageRequest, PostUserRequest, PostUserTagRequest, PutUserPasswordRequest, User,
    UserDetail, UserStats, UserTag, UserTags, Users,
};

/// `GET /users`
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
    type Error = ApiError;

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

/// `GET /users/{id}`
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
    type Error = ApiError;

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

/// `GET /users/{id}/tags`
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
    type Error = ApiError;

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

/// `PATCH /users/{id}`
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
    type Error = Infallible;

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

/// `POST /users/{id}/messages`
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
    type Error = ApiError;

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

/// `GET /users/{id}/messages`
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
    type Error = ApiError;

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

/// `GET /users/{id}/stats`
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
    type Error = ApiError;

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

/// `GET /users/{id}/icon`
#[derive(Debug, Clone)]
pub struct GetUserIcon {
    id: Uuid,
}

impl GetUserIcon {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

impl ApiRequest for GetUserIcon {
    type Response = Image;
    type Error = ApiError;

    fn uri(&self) -> String {
        format!("/users/{}/icon", self.id)
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn body(&self) -> Body {
        Body::empty()
    }

    fn parse(&self, body: Bytes) -> Result<Self::Response, Self::Error> {
        use image::io::Reader as ImageReader;

        let cursor = Cursor::new(body);
        let img = ImageReader::new(cursor).with_guessed_format()?.decode()?;
        Ok(img)
    }
}

/// `PUT /users/{id}/icon`
/// maybe works
#[derive(Debug, Clone)]
pub struct PutUserIcon {
    id: Uuid,
    request: Image,
}

impl PutUserIcon {
    pub fn new(id: Uuid, request: Image) -> Self {
        Self { id, request }
    }
}

impl ApiRequest for PutUserIcon {
    type Response = ();
    type Error = Infallible;

    fn uri(&self) -> String {
        format!("/users/{}/icon", self.id)
    }

    fn method(&self) -> Method {
        Method::PUT
    }

    fn content_type(&self) -> Option<String> {
        Some("image/png".to_string())
    }

    fn body(&self) -> Body {
        let mut buf = Cursor::new(Vec::new());
        self.request
            .write_to(&mut buf, ImageFormat::Png)
            .expect("failed to write image");
        buf.into_inner().into()
    }

    fn parse(&self, _body: Bytes) -> Result<Self::Response, Self::Error> {
        Ok(())
    }
}

/// `PUT /users/{id}/password`
/// maybe works
#[derive(Debug, Clone)]
pub struct PutUserPassword {
    id: Uuid,
    request: PutUserPasswordRequest,
}

impl PutUserPassword {
    pub fn new(id: Uuid, request: PutUserPasswordRequest) -> Self {
        Self { id, request }
    }
}

impl ApiRequest for PutUserPassword {
    type Response = ();
    type Error = Infallible;

    fn uri(&self) -> String {
        format!("/users/{}/password", self.id)
    }

    fn method(&self) -> Method {
        Method::PUT
    }

    fn content_type(&self) -> Option<String> {
        Some("application/json".to_string())
    }

    fn body(&self) -> Body {
        serde_json::to_string(&self.request)
            .expect("failed to serialize PutUserPasswordRequest")
            .into()
    }

    fn parse(&self, _body: Bytes) -> Result<Self::Response, Self::Error> {
        Ok(())
    }
}

/// `POST /users`
/// maybe works
#[derive(Debug, Clone)]
pub struct PostUser {
    request: PostUserRequest,
}

impl PostUser {
    pub fn new(request: PostUserRequest) -> Self {
        Self { request }
    }
}

impl ApiRequest for PostUser {
    type Response = UserDetail;
    type Error = ApiError;

    fn uri(&self) -> String {
        "/users".to_string()
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn content_type(&self) -> Option<String> {
        Some("application/json".to_string())
    }

    fn body(&self) -> Body {
        serde_json::to_string(&self.request)
            .expect("failed to serialize PostUserRequest")
            .into()
    }

    fn parse(&self, body: Bytes) -> Result<Self::Response, Self::Error> {
        let s = std::str::from_utf8(&body)?;
        let r = serde_json::from_str(s)?;
        Ok(r)
    }
}

/// `POST /users/{id}/tags`
#[derive(Debug, Clone)]
pub struct PostUserTag {
    id: Uuid,
    request: PostUserTagRequest,
}

impl PostUserTag {
    pub fn new(id: Uuid, request: PostUserTagRequest) -> Self {
        Self { id, request }
    }
}

impl ApiRequest for PostUserTag {
    type Response = UserTag;
    type Error = ApiError;

    fn uri(&self) -> String {
        format!("/users/{}/tags", self.id)
    }

    fn method(&self) -> Method {
        Method::POST
    }

    fn content_type(&self) -> Option<String> {
        Some("application/json".to_string())
    }

    fn body(&self) -> Body {
        serde_json::to_string(&self.request)
            .expect("failed to serialize PostUserTagRequest")
            .into()
    }

    fn parse(&self, body: Bytes) -> Result<Self::Response, Self::Error> {
        let s = std::str::from_utf8(&body)?;
        let r = serde_json::from_str(s)?;
        Ok(r)
    }
}

/// `PATCH /users/{user_id}/tags/{tag_id}`
#[derive(Debug, Clone)]
pub struct PatchUserTag {
    user_id: Uuid,
    tag_id: Uuid,
    request: PatchUserTagRequest,
}

impl PatchUserTag {
    pub fn new(user_id: Uuid, tag_id: Uuid, request: PatchUserTagRequest) -> Self {
        Self {
            user_id,
            tag_id,
            request,
        }
    }
}

impl ApiRequest for PatchUserTag {
    type Response = ();
    type Error = Infallible;

    fn uri(&self) -> String {
        format!("/users/{}/tags/{}", self.user_id, self.tag_id)
    }

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn content_type(&self) -> Option<String> {
        Some("application/json".to_string())
    }

    fn body(&self) -> Body {
        serde_json::to_string(&self.request)
            .expect("failed to serialize PatchUserTagRequest")
            .into()
    }

    fn parse(&self, _body: Bytes) -> Result<Self::Response, Self::Error> {
        Ok(())
    }
}

/// `DELETE /users/{user_id}/tags/{tag_id}`
#[derive(Debug, Clone)]
pub struct DeleteUserTag {
    user_id: Uuid,
    tag_id: Uuid,
}

impl DeleteUserTag {
    pub fn new(user_id: Uuid, tag_id: Uuid) -> Self {
        Self { user_id, tag_id }
    }
}

impl ApiRequest for DeleteUserTag {
    type Response = ();
    type Error = Infallible;

    fn uri(&self) -> String {
        format!("/users/{}/tags/{}", self.user_id, self.tag_id)
    }

    fn method(&self) -> Method {
        Method::DELETE
    }

    fn body(&self) -> Body {
        Body::empty()
    }

    fn parse(&self, _body: Bytes) -> Result<Self::Response, Self::Error> {
        Ok(())
    }
}

/// `GET /users/{id}/dm-channel`
#[derive(Debug, Clone)]
pub struct GetDirectMessageChannel {
    id: Uuid,
}

impl GetDirectMessageChannel {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

impl ApiRequest for GetDirectMessageChannel {
    type Response = DirectMessageChannel;
    type Error = ApiError;

    fn uri(&self) -> String {
        format!("/users/{}/dm-channel", self.id)
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
