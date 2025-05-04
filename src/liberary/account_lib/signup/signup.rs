use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignUpInfo {
    pub username: String,
    pub bp: u32,
    pub region: String,
    pub discord_id: String,
    pub date: u64
}
