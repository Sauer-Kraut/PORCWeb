use std::sync::Arc;
use async_std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serenity::all::UserId;


use crate::{backend::backend_api::season, liberary::{account_lib::{match_event::{match_event::{MatchEvent, MatchStatus}, storage::{get_match_event::get_match_event, store_match_event}}, signup::{signup::SignUpInfo, storage::store_signup::store_signup}}, dialogue_lib::{bot_error::BotError, dialogue_initiator::dialogue_initiator::DialogueInitator, dialogue_plan::{dialogue_data::{self, CaseData, DialogueData}, dialogue_plan::DialoguePlan, dialogue_step::{DialogueStep, StepCondition}}}, matchplan_lib::season::season::Season, util::functions::parse_user_id::parse_ds_user_id}, porcbot::{config::*, util::create_event::create_discord_event}, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeasonLeapData {
    pub new_season: Season,
}

// not perfectly efficient but makes the code a lot cleaner, plus clone sizes should be pretty small
impl TryFrom<DialogueData> for SeasonLeapData {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: DialogueData) -> Result<Self, Self::Error> {
        if let CaseData::SeasonLeap(season_leap_data) = value.data {
            Ok(season_leap_data)
        } else {
            Err("DialogueData does not contain SeasonLeapData".into())
        }
    }
}


pub fn construct_season_leap_plan(dialogue_data: DialogueData, index: u64, dialogue_id: Option<i64>) -> Result<DialoguePlan<'static>, BotError> {

    if let CaseData::SeasonLeap(_) = dialogue_data.data {

    } else {
        return Err(format!("provided dialogue data does not fit to constructer function").into())
    }

    let res = DialoguePlan {
        dialogue_id,
        index,
        dialogue_data,
        steps: vec![
            DialogueStep {  //0 So you want to sign up for the new season?

                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {

                    let info: SeasonLeapData = dialogue_data.clone().try_into()?;

                    let season_name = info.new_season.name.clone();
                    let start_timestamp = info.new_season.start_timestamp;

                    Ok(format!("Hello, as you might be aware, the next season of PORC is about to begin! \nAs of now, you have been an active member of the PORC league, which will start its next season on **<t:{start_timestamp}:F>**.\nIf you want to **continue being a part of PORC for season {season_name}**, please react with :white_check_mark: to this message.\nIf you react with :x: or do not react to this message, you will **not be entered** into PORC season {season_name}.\nThanks for participating in the last season, and we hope to see you around in the next one!\n\n**Do you want to participate in PORC season {season_name}?**"))
                })))), 

                condition: StepCondition::React(Arc::new(Mutex::new(Box::new(|reaction: Option<bool>, dialogue_data: &mut DialogueData, appstate: &AppState| Box::pin(async move {

                    let info: SeasonLeapData = dialogue_data.clone().try_into()?;
                    
                    match reaction {
                        None => {Ok(None)},

                        Some(confirmation) => {

                            if confirmation {
                                return Ok(Some(1)) // confirmation has been registered
                            }
                            else {
                                return Ok(Some(2)) // decline has been registered
                            }
                                
                        },
                    }
                })))))
            },


            // 1 singed up successfully
            DialogueStep {  
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    Ok(format!("**Your signup was registered!** \nIf you have any questions feel free to check out the PORC website or ask in the PORC discord server!"))
                })))),

                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|dialogue_data: &mut DialogueData, appstate: &AppState| Box::pin(async move {

                    store_signup(SignUpInfo { 
                        username: "See discord ID".to_string(), // this wont actually be read 
                        bp: 0, 
                        region: "NaN".to_string(), 
                        discord_id: dialogue_data.user_id.clone(), 
                        date: 0 // this wont actually be read 
                    }, appstate.pool.clone()).await?;

                    Ok(Some(600))
                })))))
            },


            // 2 your decline was registered
            DialogueStep {
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    Ok(format!("Your decline was registered. If you change your mind you can sign up via the PORC website until the start of the season."))
                })))), 

                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|dialogue_data: &mut DialogueData, appstate: &AppState| Box::pin(async move {
                    Ok(Some(600))
                })))))
            }
        ],
    };

    return Ok(res)
}