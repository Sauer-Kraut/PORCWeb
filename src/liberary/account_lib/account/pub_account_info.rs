use serde::{Deserialize, Serialize};

use crate::liberary::account_lib::{account::{account_customisation::AccountCustomisation, user_data::{radar_chart::RadarChart, shiftstone::Shiftstone, account_stats::AccountStats}}, schedule::schedule::Schedule};
use super::account::Account;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PubAccountInfo {
    pub id: String,
    pub username: String,
    pub stats: AccountStats,
    pub avatar: Option<String>,
    pub schedule: Option<Schedule>,
    pub customisation: Option<AccountCustomisation>,
}

impl Account {

    pub fn get_pub_info(&self) -> PubAccountInfo {
        PubAccountInfo {
            id: self.user_info.id.clone(),
            username: self.user_info.username.clone(),
            stats: self.stats.clone(),
            avatar:  self.user_info.avatar.clone(),
            schedule: self.schedule.clone(),
            customisation: self.customisation.clone(),
        }
    }
}