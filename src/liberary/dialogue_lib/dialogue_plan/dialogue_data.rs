use serde::{Deserialize, Serialize};

use crate::liberary::dialogue_lib::dialogue_routes::match_request::MatchRequestData;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueData {
    pub user_id: u64,
    pub data: CaseData,
    pub error: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CaseData {
    MatchRequest(MatchRequestData)
}
