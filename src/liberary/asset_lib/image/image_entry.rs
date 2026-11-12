use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ImageEntry {
    pub id: String,
    pub object_name: String,
    pub owner_id: Option<String>,
    pub author: String,
    pub link: String,
    pub filetype: String,
    pub created_at: u64,
}