use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
