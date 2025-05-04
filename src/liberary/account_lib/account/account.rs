use serde::{Deserialize, Serialize};

use crate::liberary::account_lib::schedule::schedule::Schedule;
use super::discord_user::DiscordUser;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Account {
    pub user_info: DiscordUser,
    pub schedule: Option<Schedule>
}