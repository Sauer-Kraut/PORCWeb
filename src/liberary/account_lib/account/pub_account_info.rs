use serde::{Deserialize, Serialize};

use crate::liberary::account_lib::schedule::schedule::Schedule;
use super::account::Account;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PubAccountInfo {
    pub id: u64,
    pub username: String,
    pub avatar: Option<String>,
    pub schedule: Option<Schedule>
}

impl Account {

    pub fn get_pub_info(&self) -> PubAccountInfo {
        PubAccountInfo {
            id: self.user_info.id,
            username: self.user_info.username.clone(),
            avatar:  self.user_info.avatar.clone(),
            schedule: self.schedule.clone(),
        }
    }
}