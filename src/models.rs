use image::DynamicImage;
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

/// https://github.com/traPtitech/traQ/blob/bf768fc1d4ce1d5eb1575dd64f008f70f97087dd/router/v3/users.go#L347-L352
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchUserRequest {
    pub display_name: Option<String>,
    pub twitter_id: Option<String>,
    pub role: Option<String>,
    pub state: Option<UserAccountState>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageStamp {
    pub user_id: Uuid,
    pub stamp_id: Uuid,
    pub count: u32,
    pub created_at: String,
    pub updated_at: String,
}

pub type MessageStamps = Vec<MessageStamp>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: Uuid,
    pub user_id: Uuid,
    pub channel_id: Uuid,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub pinned: bool,
    pub stamps: MessageStamps,
    pub thread_id: Option<Uuid>,
}

pub type Messages = Vec<Message>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMessageRequest {
    pub content: String,
    pub embed: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatsStamp {
    pub id: Uuid,
    pub count: i64,
    pub total: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
    pub total_message_count: i64,
    pub stamps: Vec<UserStatsStamp>,
    pub datetime: String,
}

pub type Image = DynamicImage;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutUserPasswordRequest {
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostUserRequest {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostUserTagRequest {
    pub tag: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchUserTagRequest {
    pub is_locked: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessageChannel {
    pub id: Uuid,
    pub user_id: Uuid,
}
