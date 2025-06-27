use serde::{Deserialize, Serialize};

use crate::liberary::dialogue_lib::{bot_error::BotError, dialogue_plan::{dialogue_data::{CaseData, DialogueData}, dialogue_plan::DialoguePlan}, dialogue_routes::{info::construct_info_plan, match_request::construct_match_request_plan, season_invite_prompt::construct_season_invite_plan, season_leap_prompt::construct_season_leap_plan}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DialogueBuilder {
    pub dialogue_id: Option<i64>,
    pub dialogue_data: DialogueData,
    pub index: u64
}

impl DialogueBuilder {

    pub async fn build<'a>(self) -> Result<DialoguePlan<'static>, BotError> {

        match &self.dialogue_data.data {
            CaseData::MatchRequest(_) => construct_match_request_plan(self.dialogue_data, self.index, self.dialogue_id),
            CaseData::Info(_) => construct_info_plan(self.dialogue_data, self.index, self.dialogue_id),
            CaseData::SeasonLeap(_) => construct_season_leap_plan(self.dialogue_data, self.index, self.dialogue_id),
            CaseData::SeasonInvite(_) => construct_season_invite_plan(self.dialogue_data, self.index, self.dialogue_id),
        }
    }
}
