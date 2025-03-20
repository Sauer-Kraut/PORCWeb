use async_std::sync::Mutex;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

use super::dialogue_data::DialogueData;
use crate::{porcbot::util::{reaction_check::*, response_check::*}, AppState};

#[derive(Clone)]
pub struct DialogueStep<'a> {
    pub message: Arc<Mutex<Box<dyn for<'c> Fn(&'c DialogueData) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'c>> + Send + 'a>>>,
    pub condition: StepCondition<'a>,
}

impl<'a, 'b> DialogueStep<'a> {
    pub async fn check_completion(&self, dialogue_data: &mut DialogueData, app_state: &AppState) -> Result<Option<u64>, String> {
        match self.condition.clone() {
            StepCondition::Info(script) => {
                return script.lock().await(dialogue_data, app_state).await;
            },
            StepCondition::React(script) => {
                let check = check_reaction(dialogue_data.user_id, self.get_message(&dialogue_data).await?).await?;
                return script.lock().await(check, dialogue_data, app_state).await
            },
            StepCondition::Response(script) => {
                let check = check_response(dialogue_data.user_id, self.get_message(&dialogue_data).await?).await?;
                return script.lock().await(check, dialogue_data, app_state).await
            },
            StepCondition::Custom(check, script) => {
                let check = (check.lock().await)(dialogue_data).await?;
                return script.lock().await(check, dialogue_data, app_state).await
            },
        }
    }

    pub async fn get_message(&self, dialogue_data: &DialogueData) -> Result<String, String> {
        (self.message.lock().await)(dialogue_data).await
    }

    pub fn default_error() -> DialogueStep<'a> {
        DialogueStep {  //400 error occurred
            message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                let error = match dialogue_data.error.clone() {
                    None => {
                        return Ok("Oh no, looks like an unidentified error occurred while processing your conversation. You might want to contact a mod about this one, in theory this shouldnt be possible".to_string())
                    },
                    Some(err) => err
                };
                Ok(format!("Oh no, an error occurred while processing one of our conversations! Sory for that, here is the error:
{error}"))
            })))), 
            condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|_dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                Ok(Some(600))
            })))))
        }
    }
}

#[derive(Clone)]
pub enum StepCondition<'a> {
    Info(Arc<Mutex<Box<dyn for<'c> Fn(&'c mut DialogueData, &'c AppState) -> Pin<Box<dyn Future<Output = Result<Option<u64>, String>> + Send + 'c>> + Send + 'a>>>),
    React(Arc<Mutex<Box<dyn for<'c> Fn(Option<bool>, &'c mut DialogueData, &'c AppState) -> Pin<Box<dyn Future<Output = Result<Option<u64>, String>> + Send + 'c>> + Send + 'a>>>),
    Response(Arc<Mutex<Box<dyn for<'c> Fn(Option<String>, &'c mut DialogueData, &'c AppState) -> Pin<Box<dyn Future<Output = Result<Option<u64>, String>> + Send + 'c>> + Send + 'a>>>),
    Custom(
        Arc<Mutex<Box<dyn for<'c> Fn(&'c DialogueData) -> Pin<Box<dyn Future<Output = Result<Option<bool>, String>> + Send + 'c>> + Send + 'a>>>,
        Arc<Mutex<Box<dyn for<'c> Fn(Option<bool>, &'c mut DialogueData, &'c AppState) -> Pin<Box<dyn Future<Output = Result<Option<u64>, String>> + Send + 'c>> + Send + 'a>>>
    )
}