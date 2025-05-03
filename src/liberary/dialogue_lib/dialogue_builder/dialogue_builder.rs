use serde::{Deserialize, Serialize};

use crate::liberary::dialogue_lib::{dialogue_plan::{dialogue_data::{CaseData, DialogueData}, dialogue_plan::DialoguePlan}, dialogue_routes::match_request::construct_match_request_plan};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueBuilder {
    pub dialogue_id: Option<i64>,
    pub dialogue_data: DialogueData,
    pub index: u64
}

impl DialogueBuilder {

    pub async fn build<'a>(self) -> Result<DialoguePlan<'static>, String> {

        match &self.dialogue_data.data {
            CaseData::MatchRequest(match_request_data) => construct_match_request_plan(self.dialogue_data, self.index, self.dialogue_id),
        }
    }
}
