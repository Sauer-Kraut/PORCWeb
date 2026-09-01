use serde::{Deserialize, Serialize};

use crate::liberary::account_lib::{account::{account_customisation::AccountCustomisation, user_data::{radar_chart::RadarChart, shiftstone::Shiftstone, account_stats::AccountStats}}, schedule::schedule::Schedule};
use super::discord_user::DiscordUser;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Account {
    pub user_info: DiscordUser,
    pub stats: AccountStats,
    pub schedule: Option<Schedule>,
    pub customisation: Option<AccountCustomisation>,
}