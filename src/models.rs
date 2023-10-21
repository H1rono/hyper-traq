use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

/// https://github.com/traPtitech/traQ/blob/bf768fc1d4ce1d5eb1575dd64f008f70f97087dd/router/v3/responses.go#L79-L87
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: Uuid,
    name: String,
    display_name: String,
    icon_file_id: Uuid,
    bot: bool,
    state: i32,
    updated_at: String,
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
    tag_id: Uuid,
    tag: String,
    is_locked: bool,
    created_at: String,
    updated_at: String,
}

/// https://github.com/traPtitech/traQ/blob/bf768fc1d4ce1d5eb1575dd64f008f70f97087dd/router/v3/responses.go#L110-L124
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    id: Uuid,
    state: UserAccountState,
    bot: bool,
    icon_file_id: Uuid,
    display_name: String,
    name: String,
    twitter_id: String,
    last_online: Option<String>,
    updated_at: String,
    tags: Vec<UserTag>,
    groups: Vec<Uuid>,
    bio: String,
    home_channel: Option<Uuid>,
}
