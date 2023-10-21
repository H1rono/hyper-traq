use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

/// https://github.com/traPtitech/traQ/blob/bf768fc1d4ce1d5eb1575dd64f008f70f97087dd/router/v3/responses.go#L79-L87
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub icon_file_id: Uuid,
    pub bot: bool,
    pub state: i32,
    pub updated_at: String,
}

pub type Users = Vec<User>;

/// https://github.com/traPtitech/traQ/blob/bf768fc1d4ce1d5eb1575dd64f008f70f97087dd/model/users.go#L41-L48
#[derive(Debug, Clone, PartialEq, Eq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UserAccountState {
    Deactivated = 0,
    Active = 1,
    Suspended = 2,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTag {
    pub tag_id: Uuid,
    pub tag: String,
    pub is_locked: bool,
    pub created_at: String,
    pub updated_at: String,
}

pub type UserTags = Vec<UserTag>;

/// https://github.com/traPtitech/traQ/blob/bf768fc1d4ce1d5eb1575dd64f008f70f97087dd/router/v3/responses.go#L110-L124
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    pub id: Uuid,
    pub state: UserAccountState,
    pub bot: bool,
    pub icon_file_id: Uuid,
    pub display_name: String,
    pub name: String,
    pub twitter_id: String,
    pub last_online: Option<String>,
    pub updated_at: String,
    pub tags: UserTags,
    pub groups: Vec<Uuid>,
    pub bio: String,
    pub home_channel: Option<Uuid>,
}
