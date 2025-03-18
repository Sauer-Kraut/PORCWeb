use std::mem;

use crate::{backend::account_lib::MatchEvent, porcbot::{dialogue_routes::match_request::MatchRequestData, util::{dm_send::*, prompt_message_send::*}}, AppState};
use super::{dialogue_builder::DialogueBuilder, dialogue_data::{self, *}, dialogue_step::*};



#[derive(Clone)]
pub struct DialoguePlan<'a> {
    pub index: u64,
    pub steps: Vec<DialogueStep<'a>>,
    pub dialogue_data: DialogueData
}

impl <'a, 'b> DialoguePlan<'a> {

    pub async fn check(&mut self, app_state: &AppState) -> Result<bool, String> {
        let current_step = match self.steps.get(self.index as usize) {
            Some(val) => val,
            None => return Err("current step could not be found".to_string()),
        };

        // Scope the mutable borrow to the block
        let next_index = current_step.check_completion(&mut self.dialogue_data, app_state).await?;

        // Now you can call `next` without conflicting borrows
        match next_index {
            Some(index) => {
                self.index = index; // No need to borrow mutably here anymore
                self.next(index).await?; // `next()` doesn't borrow `self.dialogue_data` mutably
                Ok(true)
            },
            None => Ok(false),
        }
    }

    async fn next(&mut self, target_index: u64) -> Result<(), String> {
        if target_index == 600 {
            println!("A dialogue has reached its end");
            Ok(())
        } else {
            if let Some(step) = self.steps.get(target_index as usize) {
                match step.condition {
                    StepCondition::React(_) => {
                        let _ = send_prompt_dm(self.dialogue_data.user_id, step.get_message(&self.dialogue_data).await?).await?;
                    },
                    _ => {
                        let _ = send_dm(self.dialogue_data.user_id, step.get_message(&self.dialogue_data).await?).await?;
                    },
                }
                self.index = target_index; // No mutable borrow of `self.dialogue_data` here
                Ok(())
            } else {
                Err("couldn't find next step".to_string())
            }
        }
    }

    pub fn get_builder(self) -> DialogueBuilder {
        DialogueBuilder {
            dialogue_data: self.dialogue_data,
            index: self.index,
        }
    }
}