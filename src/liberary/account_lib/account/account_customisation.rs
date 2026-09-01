use serde::{Serialize, Deserialize};

use crate::liberary::account_lib::account::user_data::{radar_chart::RadarChart, shiftstone::Shiftstone};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AccountCustomisation {
    pub region: Option<String>,
    pub bp: u32,
    pub radar_chart: Option<RadarChart>,
    pub shiftstones: (Option<Shiftstone>, Option<Shiftstone>),
    pub badges: Vec<u64>,
    pub unlocked_badges: Vec<u64>,
    pub banner: Option<String>
}