use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    name: String,
    display_name: String,
    icon_file_id: String,
    bot: bool,
    state: i32,
    updated_at: String,
}

pub type Users = Vec<User>;
