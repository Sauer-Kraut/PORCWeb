use std::mem;

use colored::Colorize;

use crate::{liberary::dialogue_lib::{bot_error::BotError, dialogue_builder::dialogue_builder::DialogueBuilder}, porcbot::util::{dm_send::send_dm, prompt_message_send::send_prompt_dm}, AppState};

use super::{dialogue_data::DialogueData, dialogue_step::{DialogueStep, StepCondition}};



#[derive(Clone)]
pub struct DialoguePlan<'a> {
    pub dialogue_id: Option<i64>,
    pub index: u64,
    pub steps: Vec<DialogueStep<'a>>,
    pub dialogue_data: DialogueData
}

impl <'a, 'b> DialoguePlan<'a> {

    pub async fn check(&mut self, app_state: &AppState) -> Result<bool, BotError> {

        // Scope the mutable borrow to the block
        let res_next_index = match self.index {
            100 => {
                Ok(Some(0 as u64))
            }
            _ => {
                let current_step = match self.steps.get(self.index as usize) {
                    Some(val) => val,
                    None => return Err("current step could not be found".to_string().into()),
                };
                current_step.check_completion(&mut self.dialogue_data, app_state).await
            }
        };

        let next_index = match res_next_index {
            Ok(i) => i,
            Err(err) => {
                println!("{}\n{}", "An error occured while checking a dialogue:".red(), err.to_string().bright_red());

                let error = err.to_string();

                // !!!!!!!!!!!!!!!!!!
                // TODO: report to DB
                // !!!!!!!!!!!!!!!!!!

                // Return None so that step may be repeated in the future
                let mut res = None;

                if let Some(prev_err) = &self.dialogue_data.error {
                    if *prev_err == error {
                        println!("{}\n{}", "Stoping Dialgogue because of recouring error:".red(), prev_err.bright_red());
                        self.dialogue_data.error = Some(err.to_string());
                        res = Some(400);
                    } 
                }

                self.dialogue_data.error = Some(err.to_string());
                res
            },
        };

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

    async fn next(&mut self, target_index: u64) -> Result<(), BotError> {
        match target_index {
            600 => {
                println!("A dialogue has reached its end");
                return Ok(())
            }
            400 => {
                let error_step = DialogueStep::default_error();
                let _ = send_dm(self.dialogue_data.user_id.clone(), error_step.get_message(&self.dialogue_data).await?).await?;
                return Ok(())
            }
            _ => {
                if let Some(step) = self.steps.get(target_index as usize) {
                    match step.condition {
                        StepCondition::React(_) => {
                            let _ = send_prompt_dm(self.dialogue_data.user_id.clone(), step.get_message(&self.dialogue_data).await?).await?;
                        },
                        _ => {
                            let _ = send_dm(self.dialogue_data.user_id.clone(), step.get_message(&self.dialogue_data).await?).await?;
                        },
                    }
                    self.index = target_index; // No mutable borrow of `self.dialogue_data` here
                    Ok(())
                } else {
                    Err("couldn't find next step".to_string().into())
                }
            }
        }
    }

    pub fn get_builder(self) -> DialogueBuilder {
        DialogueBuilder {
            dialogue_id: self.dialogue_id,
            dialogue_data: self.dialogue_data,
            index: self.index,
        }
    }
}