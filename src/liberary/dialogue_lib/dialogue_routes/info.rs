use std::sync::Arc;
use async_std::sync::Mutex;
use serde::{Deserialize, Serialize};

use crate::liberary::dialogue_lib::bot_error::BotError;
use crate::liberary::dialogue_lib::dialogue_plan::dialogue_data::{CaseData, DialogueData};
use crate::liberary::dialogue_lib::dialogue_plan::dialogue_plan::DialoguePlan;
use crate::liberary::dialogue_lib::dialogue_plan::dialogue_step::{DialogueStep, StepCondition};
use crate::AppState;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoData {
    pub message: String
}

// not perfectly efficient but makes the code a lot cleaner, plus clone sizes should be pretty small
impl TryFrom<DialogueData> for InfoData {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: DialogueData) -> Result<Self, Self::Error> {
        if let CaseData::Info(match_request_data) = value.data {
            Ok(match_request_data)
        } else {
            Err("DialogueData does not contain MatchRequestData".into())
        }
    }
}


pub fn construct_info_plan(dialogue_data: DialogueData, index: u64, dialogue_id: Option<i64>) -> Result<DialoguePlan<'static>, BotError> {

    if let CaseData::Info(_) = dialogue_data.data {

    } else {
        return Err(format!("provided dialogue data does not fit to constructer function").into())
    }

    let res = DialoguePlan {
        dialogue_id,
        index,
        dialogue_data,
        steps: vec![
            DialogueStep {  // 0 Send message to user, immidiatly finishes afterwards

                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {

                    let info: InfoData = dialogue_data.clone().try_into()?;
                    Ok(info.message)
                })))), 

                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|_dialogue_data: &mut DialogueData, _appstate: &AppState| Box::pin(async move {
                    Ok(Some(600)) // 600 means dialogue end
                })))))
            }]
    };

    return Ok(res)
}