use crate::{liberary::{account_lib::match_event::match_event::MatchEvent, dialogue_lib::{bot_error::BotError, dialogue_builder::dialogue_builder::DialogueBuilder, dialogue_plan::{dialogue_data::{CaseData, DialogueData}, dialogue_step::StepCondition}, dialogue_routes::match_request::MatchRequestData}}, porcbot::util::{dm_send::send_dm, prompt_message_send::send_prompt_dm}};


#[derive(Copy, Clone)]
pub struct DialogueInitator {}

impl DialogueInitator {

    pub async fn initiate_match_request<'a>(user_id: String, division_name: String, match_info: MatchEvent) -> Result<DialogueBuilder, BotError> {

        let builder = DialogueBuilder {
            dialogue_id: None,
            dialogue_data: DialogueData {
                user_id: user_id.clone(),
                data: CaseData::MatchRequest(MatchRequestData {
                    match_info,
                    division_name,
                    event_id: None,
                }),
                error: None
            },
            index: 0
        };

        let plan = builder.build().await?;

        let first_step = match plan.steps.first() {
            Some(v) => v,
            None => return Err("plan does not have steps".to_string().into()),
        };

        match first_step.condition {
            StepCondition::React(_) => {let _ = send_prompt_dm(user_id, first_step.get_message(&plan.dialogue_data).await?).await;},
            _ => {let _ = send_dm(user_id, first_step.get_message(&plan.dialogue_data).await?).await;}
        }

        Ok(plan.get_builder())
    }
}
