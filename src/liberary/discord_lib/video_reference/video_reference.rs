use serde::{Deserialize, Serialize};

use crate::liberary::account_lib::account::pub_account_info::PubAccountInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoReference {
    pub title: String,
    pub youtube_id: String,
    pub creator: PubAccountInfo,
    pub date: u64,
}