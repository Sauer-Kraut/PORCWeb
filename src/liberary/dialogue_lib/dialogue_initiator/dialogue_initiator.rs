use crate::{liberary::{account_lib::match_event::match_event::MatchEvent, dialogue_lib::{bot_error::BotError, dialogue_builder::{dialogue_builder::DialogueBuilder, storage::store_dialogue}, dialogue_plan::{dialogue_data::{CaseData, DialogueData}, dialogue_step::StepCondition}, dialogue_routes::{info::InfoData, match_request::MatchRequestData, season_invite_prompt::SeasonInviteData, season_leap_prompt::SeasonLeapData}}, matchplan_lib::season::season::Season}, porcbot::util::{dm_send::send_dm, prompt_message_send::send_prompt_dm}, AppState};


#[derive(Copy, Clone)]
pub struct DialogueInitator {}

// Adds a bit of overhead and extra complexity, but makes it more idiot proof in return

impl DialogueInitator {

    async fn initiate_dialogue(appstate: &AppState, user_id: String, data: CaseData) -> Result<DialogueBuilder, BotError> {

        let builder = DialogueBuilder {
            dialogue_id: None,
            dialogue_data: DialogueData {
                user_id: user_id.clone(),
                data,
                error: None
            },
            index: 100
        };

        let res = store_dialogue::store_dialogue(builder.clone(), appstate.pool.clone()).await;
        match res {
            Ok(_) => {},
            Err(err) => {
                return Err(format!("Error while storing dialogue: {:?}", err).into());
            }
        };

        Ok(builder)
    }

    pub async fn initiate_match_request<'a>(appstate: &AppState, user_id: String, division_name: String, match_info: MatchEvent) -> Result<DialogueBuilder, BotError> {

        let data = CaseData::MatchRequest(MatchRequestData {
            match_info,
            division_name,
            event_id: None,
        });

        Self::initiate_dialogue(appstate, user_id, data).await
    }


    pub async fn initiate_info<'a>(appstate: &AppState, user_id: String, message: String) -> Result<DialogueBuilder, BotError> {

        let data = CaseData::Info(InfoData {
            message
        });

        Self::initiate_dialogue(appstate, user_id, data).await
    }


    pub async fn initiate_season_leap<'a>(appstate: &AppState, user_id: String, new_season: Season) -> Result<DialogueBuilder, BotError> {

        let data = CaseData::SeasonLeap(SeasonLeapData {
            new_season
        });

        Self::initiate_dialogue(appstate, user_id, data).await
    }


    pub async fn initiate_season_invite<'a>(appstate: &AppState, user_id: String, new_season: Season) -> Result<DialogueBuilder, BotError> {

        let data = CaseData::SeasonInvite(SeasonInviteData {
            new_season,
            bp: None
        });

        Self::initiate_dialogue(appstate, user_id, data).await
    }

    
}
