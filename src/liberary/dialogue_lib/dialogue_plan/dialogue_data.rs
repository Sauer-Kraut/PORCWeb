use serde::{Deserialize, Serialize};

use crate::liberary::dialogue_lib::dialogue_routes::{info::InfoData, match_request::MatchRequestData};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueData {
    pub user_id: String,
    pub data: CaseData,
    pub error: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CaseData {
    MatchRequest(MatchRequestData),
    Info(InfoData)
}
