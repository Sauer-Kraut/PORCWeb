use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AccountStats {
    pub wins: u32,
    pub matches: u32,
    pub win_ratio: Option<f64>,
    pub global_rank: Option<u32>,
    pub division_rank: Option<u32>,
}