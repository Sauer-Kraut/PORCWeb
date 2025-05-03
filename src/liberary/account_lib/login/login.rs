use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LogIn {
    pub key: String,
    pub account_id: String,
    pub creation_timestamp: u64,
}