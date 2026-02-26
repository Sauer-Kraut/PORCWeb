use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscordEvent {
    pub title: String,
    pub description: String,
    pub img_id: String,
    pub interested: u32,
    pub link: String,
    pub start_time: u64,
    pub place: String,
    pub live: bool
}