use std::cmp::{PartialEq, Eq};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MatchEvent {
    pub id: Option<i32>, // Not meant to be used in order to store in databank, solely for identification among shedule
    pub start_timestamp: u64,
    pub challenger_id: u64,
    pub opponent_id: u64,
    pub event_id: Option<u64>,
    pub status: MatchStatus,
    pub season: String
}

impl MatchEvent{

    pub fn get_id(&self) -> String {
        let mut lower_id = self.challenger_id.clone();
        let mut higher_id = self.opponent_id.clone();

        if lower_id > higher_id {
            lower_id = self.opponent_id.clone();
            higher_id = self.challenger_id.clone();
        }

        format!("{}V{}@{}", lower_id, higher_id, self.start_timestamp)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MatchStatus {
    Requested,
    Confirmed,
    Finished,
    Declined
}

impl MatchStatus {
    pub fn from_status_code(code: i16) -> Result<MatchStatus, String> {
        match code {
            0 => Ok(MatchStatus::Requested),
            1 => Ok(MatchStatus::Confirmed),
            2 => Ok(MatchStatus::Finished),
            3 => Ok(MatchStatus::Declined),
            _ => Err("Invalid status code".to_string())
        }
    }
}

impl PartialEq for MatchEvent {
    fn eq(&self, other: &Self) -> bool {
        // Compare relevant fields for equality
        self.start_timestamp == other.start_timestamp
            && self.season == other.season
            && ((self.challenger_id == other.challenger_id && self.opponent_id == other.opponent_id)
                || (self.challenger_id == other.opponent_id && self.opponent_id == other.challenger_id))
    }
}

impl Eq for MatchEvent {} // Required for types implementing PartialEq
