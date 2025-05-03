use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DiscordUser {
    pub id: u64,
    pub username: String,
    pub discriminator: i8,
    pub avatar: Option<String>,
    pub email: Option<String>,
}