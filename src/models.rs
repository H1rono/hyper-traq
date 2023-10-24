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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StampHistoryEntry {
    pub stamp_id: Uuid,
    pub datetime: String,
}

pub type StampHistoryEntries = Vec<StampHistoryEntry>;

#[derive(Debug, Clone)]
pub enum QrCode {
    Image(Image),
    Text(String),
}

impl QrCode {
    pub fn image(self) -> Option<Image> {
        match self {
            Self::Image(image) => Some(image),
            Self::Text(_) => None,
        }
    }

    pub fn text(self) -> Option<String> {
        match self {
            Self::Image(_) => None,
            Self::Text(text) => Some(text),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserPermission {
    GetWebhook,
    CreateWebhook,
    EditWebhook,
    DeleteWebhook,
    AccessOthersWebhook,
    GetBot,
    CreateBot,
    EditBot,
    DeleteBot,
    AccessOthersBot,
    BotActionJoinChannel,
    BotActionLeaveChannel,
    CreateChannel,
    GetChannel,
    EditChannel,
    DeleteChannel,
    ChangeParentChannel,
    EditChannelTopic,
    GetChannelStar,
    EditChannelStar,
    GetMyTokens,
    RevokeMyToken,
    GetClients,
    CreateClient,
    EditMyClient,
    DeleteMyClient,
    ManageOthersClient,
    UploadFile,
    DownloadFile,
    DeleteFile,
    GetMessage,
    PostMessage,
    EditMessage,
    DeleteMessage,
    ReportMessage,
    GetMessageReports,
    CreateMessagePin,
    DeleteMessagePin,
    GetChannelSubscription,
    EditChannelSubscription,
    ConnectNotificationStream,
    RegisterFcmDevice,
    GetStamp,
    CreateStamp,
    EditStamp,
    EditStampCreatedByOthers,
    DeleteStamp,
    AddMessageStamp,
    RemoveMessageStamp,
    GetMyStampHistory,
    GetStampPalette,
    CreateStampPalette,
    EditStampPalette,
    DeleteStampPalette,
    GetUser,
    RegisterUser,
    GetMe,
    EditMe,
    ChangeMyIcon,
    ChangeMyPassword,
    EditOtherUsers,
    GetUserQrCode,
    GetUserTag,
    EditUserTag,
    GetUserGroup,
    CreateUserGroup,
    CreateSpecialUserGroup,
    EditUserGroup,
    DeleteUserGroup,
    EditOthersUserGroup,
    WebRtc,
    GetMySessions,
    DeleteMySessions,
    GetMyExternalAccount,
    EditMyExternalAccount,
    GetUnread,
    DeleteUnread,
    GetClipFolder,
    CreateClipFolder,
    EditClipFolder,
    DeleteClipFolder,
}

pub type UserPermissions = Vec<UserPermission>;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MyUserDetail {
    pub id: Uuid,
    pub bio: String,
    pub groups: Vec<Uuid>,
    pub tags: UserTags,
    pub updated_at: String,
    pub last_online: Option<String>,
    pub twitter_id: String,
    pub name: String,
    pub display_name: String,
    pub icon_file_id: Uuid,
    pub bot: bool,
    pub state: UserAccountState,
    pub permissions: UserPermissions,
    pub home_channel: Option<Uuid>,
}
