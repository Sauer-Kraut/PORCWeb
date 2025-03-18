use crate::{backend::account_lib::MatchEvent, porcbot::{dialogue_routes::match_request::{construct_match_request_plan, MatchRequestData}, util::{dm_send::send_dm, prompt_message_send::send_prompt_dm}}};
use super::{dialogue_data::*, dialogue_plan::DialoguePlan, dialogue_builder::*};

#[derive(Copy, Clone)]
pub struct DialogueInitator {}

impl DialogueInitator {

    pub async fn initiate_match_request<'a>(user_id: u64, division_name: String, match_info: MatchEvent) -> Result<DialogueBuilder, String> {

        let builder = DialogueBuilder {
            dialogue_data: DialogueData {
                user_id,
                data: CaseData::MatchRequest(MatchRequestData {
                    match_info,
                    division_name,
                    event_id: None,
                })
            },
            index: 0
        };

        let plan = builder.build().await?;

        let first_step = match plan.steps.first() {
            Some(v) => v,
            None => return Err("plan does not have steps".to_string()),
        };

        match first_step.condition {
            super::dialogue_step::StepCondition::React(_) => {let _ = send_prompt_dm(user_id, first_step.get_message(&plan.dialogue_data).await?).await;},
            _ => {let _ = send_dm(user_id, first_step.get_message(&plan.dialogue_data).await?).await;}
        }

        Ok(plan.get_builder())
    }
}
