use hyper::body::Bytes;
use hyper::{Body, Method};

use super::{ApiError, ApiRequest};
use crate::models::{MyUserDetail, StampHistoryEntries};

/// `GET /users/me/stamp-history`
#[derive(Debug, Clone, Default)]
pub struct GetMyStampHistory {
    limit: Option<i64>,
}

impl GetMyStampHistory {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl ApiRequest for GetMyStampHistory {
    type Response = StampHistoryEntries;
    type Error = ApiError;

    fn uri(&self) -> String {
        if let Some(limit) = self.limit {
            return format!("/users/me/stamp-history?limit={}", limit);
        }
        "/users/me/stamp-history".to_string()
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn accept(&self) -> Option<String> {
        Some("application/json".to_string())
    }

    fn body(&self) -> Body {
        Body::empty()
    }

    fn parse(&self, body: Bytes) -> Result<Self::Response, Self::Error> {
        let s = std::str::from_utf8(&body)?;
        let v = serde_json::from_str(s)?;
        Ok(v)
    }
}

/// `GET /users/me`
#[derive(Debug, Clone, Default)]
pub struct GetMe;

impl GetMe {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ApiRequest for GetMe {
    type Response = MyUserDetail;
    type Error = ApiError;

    fn uri(&self) -> String {
        "/users/me".to_string()
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn accept(&self) -> Option<String> {
        Some("application/json".to_string())
    }

    fn body(&self) -> Body {
        Body::empty()
    }

    fn parse(&self, body: Bytes) -> Result<Self::Response, Self::Error> {
        let s = std::str::from_utf8(&body)?;
        let v = serde_json::from_str(s)?;
        Ok(v)
    }
}
