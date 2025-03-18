use serde::{Deserialize, Serialize};

use crate::{backend::account_lib::MatchEvent, porcbot::dialogue_routes::match_request::MatchRequestData};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueData {
    pub user_id: u64,
    pub data: CaseData
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CaseData {
    MatchRequest(MatchRequestData)
}
