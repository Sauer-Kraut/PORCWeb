use serde::{Deserialize, Serialize};
use std::fmt;

use crate::liberary::matchplan_lib::{division::division::Division, player::player::Player};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchPlan {
    pub divisions: Vec<Division>,
    pub players: Vec<Player>,
    pub end_timestamp: u64,
    pub pause_end_timestamp: u64,
    pub season: String
}

impl fmt::Display for MatchPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MatchPlan:\nDivisions:\n")?;
        for division in &self.divisions {
            write!(f, "{}\n", division)?;
        }
        write!(f, "Players:\n")?;
        for player in &self.players {
            write!(f, "{}\n", player)?;
        }
        Ok(())
    }
}