use std::sync::Arc;
use async_std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serenity::all::UserId;


use crate::{backend::backend_api::season, liberary::{account_lib::{match_event::{match_event::{MatchEvent, MatchStatus}, storage::{get_match_event::get_match_event, store_match_event}}, signup::{signup::SignUpInfo, storage::store_signup::store_signup}}, dialogue_lib::{bot_error::BotError, dialogue_initiator::dialogue_initiator::DialogueInitator, dialogue_plan::{dialogue_data::{self, CaseData, DialogueData}, dialogue_plan::DialoguePlan, dialogue_step::{DialogueStep, StepCondition}}}, matchplan_lib::season::season::Season, util::functions::parse_user_id::parse_ds_user_id}, porcbot::{config::*, util::create_event::create_discord_event}, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeasonInviteData {
    pub new_season: Season,
    pub bp: Option<u64>
}

// not perfectly efficient but makes the code a lot cleaner, plus clone sizes should be pretty small
impl TryFrom<DialogueData> for SeasonInviteData {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: DialogueData) -> Result<Self, Self::Error> {
        if let CaseData::SeasonInvite(season_invite_data) = value.data {
            Ok(season_invite_data)
        } else {
            Err("DialogueData does not contain SeasonInviteData".into())
        }
    }
}


pub fn construct_season_invite_plan(dialogue_data: DialogueData, index: u64, dialogue_id: Option<i64>) -> Result<DialoguePlan<'static>, BotError> {

    if let CaseData::SeasonInvite(_) = dialogue_data.data {

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

                    let info: SeasonInviteData = dialogue_data.clone().try_into()?;

                    let season_name = info.new_season.name.clone();
                    let start_timestamp = info.new_season.start_timestamp;

                    Ok(format!("Hello, as you might be aware, the next season of PORC is about to begin!\nYou are receiving this message because you currently have the competitor role but are not competing in the PORC league.\nIf you want to **sign up for PORC season {season_name}**, which will start **<t:{start_timestamp}:F>**, please react with :white_check_mark: to this message.\nIf you react with :x: or do not react, you will **not be entered** into the PORC season {season_name}.\n\n**Do you want to participate in PORC season {season_name}?**"))
                })))), 

                condition: StepCondition::React(Arc::new(Mutex::new(Box::new(|reaction: Option<bool>, dialogue_data: &mut DialogueData, appstate: &AppState| Box::pin(async move {

                    let info: SeasonInviteData = dialogue_data.clone().try_into()?;
                    
                    match reaction {
                        None => {Ok(None)},

                        Some(confirmation) => {

                            if confirmation {
                                return Ok(Some(1)) // what is your bp?
                            }
                            else {
                                return Ok(Some(4)) // decline has been registered
                            }
                                
                        },
                    }
                })))))
            },


            // 1 what is your bp?
            DialogueStep {  
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    Ok(format!("What is your current bp? (a rough estimate is fine, please awnser with a single number, e.g. 6200)"))
                })))),

                condition: StepCondition::Response(Arc::new(Mutex::new(Box::new(|response: Option<String>, dialogue_data: &mut DialogueData, appstate: &AppState| Box::pin(async move {

                    let info: &mut SeasonInviteData = match dialogue_data.data {
                        CaseData::SeasonInvite(ref mut data) => data,
                        _ => return Err("Dialogue Route has incorrect Case data".to_string().into())
                    };

                    match response {
                        None => {Ok(None)},

                        Some(bp) => {
                            // try to parse the bp
                            let bp: u64 = match bp.parse() {
                                Ok(bp) => bp,
                                Err(_) => return Ok(Some(2)) // couldnt parse bp, please try again
                            };
                            
                            info.bp = Some(bp);
                            return Ok(Some(3)) // signup has been registered
                        },
                    }
                })))))
            },


            // 2 Failed to parse bp, please try again
            DialogueStep {  
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    Ok(format!("Sorry, but I wasnt able to parse your BP. Please try again and make sure to **only send your BP as a plain number** (do not write 'BP' behind your message, do not shorten thousands with 'k'). \nIf you continue to run into issues feel free to contact one of our mods or try signing up via the PORC website."))
                })))),

                condition: StepCondition::Response(Arc::new(Mutex::new(Box::new(|response: Option<String>, dialogue_data: &mut DialogueData, appstate: &AppState| Box::pin(async move {

                    let info: &mut SeasonInviteData = match dialogue_data.data {
                        CaseData::SeasonInvite(ref mut data) => data,
                        _ => return Err("Dialogue Route has incorrect Case data".to_string().into())
                    };

                    match response {
                        None => {Ok(None)},

                        Some(bp) => {
                            // try to parse the bp
                            let bp: u64 = match bp.parse() {
                                Ok(bp) => bp,
                                Err(_) => return Ok(Some(2)) // couldnt parse bp, please try again
                            };

                            info.bp = Some(bp);
                            return Ok(Some(3)) // signup has been registered
                        },
                    }
                })))))
            },


            // 3 signup has been registered
            DialogueStep {  
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    Ok(format!("**Your signup was registered!** \nIf you have any questions feel free to check out the PORC website or ask in the PORC discord server!"))
                })))),

                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|dialogue_data: &mut DialogueData, appstate: &AppState| Box::pin(async move {

                    let info: SeasonInviteData = dialogue_data.clone().try_into()?;

                    store_signup(SignUpInfo { 
                        username: "See discord ID".to_string(), // this wont actually be read 
                        bp: info.bp.unwrap_or(0) as u32, 
                        region: "NaN".to_string(), 
                        discord_id: dialogue_data.user_id.clone(), 
                        date: 0 // this wont actually be read 
                    }, appstate.pool.clone()).await?;

                    Ok(Some(600))
                })))))
            },


            // 4 your decline was registered
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