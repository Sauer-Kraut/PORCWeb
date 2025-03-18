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