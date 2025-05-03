use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LogIn {
    pub key: String,
    pub account_id: u64,
    pub creation_timestamp: u64,
}