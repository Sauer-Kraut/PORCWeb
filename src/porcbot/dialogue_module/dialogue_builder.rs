use serde::{Deserialize, Serialize};
use crate::porcbot::dialogue_routes::match_request::construct_match_request_plan;

use super::{dialogue_data::*, dialogue_plan::DialoguePlan};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueBuilder {
    pub dialogue_data: DialogueData,
    pub index: u64
}

impl DialogueBuilder {

    pub async fn build<'a>(self) -> Result<DialoguePlan<'static>, String> {

        match &self.dialogue_data.data {
            CaseData::MatchRequest(match_request_data) => construct_match_request_plan(self.dialogue_data, self.index),
        }
    }
}
