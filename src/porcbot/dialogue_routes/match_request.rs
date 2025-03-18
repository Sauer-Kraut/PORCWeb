use std::sync::Arc;

use actix_web::web;
use async_std::sync::Mutex;
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use serenity::all::{User, UserId};

use crate::{backend::account_lib::{put_match_event, MatchEvent, MatchStatus, PutMatchEventRecvPackage}, porcbot::{config::get_http, dialogue_module::{dialogue_data::{self, CaseData, DialogueData}, dialogue_plan::DialoguePlan, dialogue_step::{DialogueStep, StepCondition}}, util::create_event::create_discord_event}, AppState};
use crate::porcbot::config::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchRequestData {
    pub match_info: MatchEvent,
    pub division_name: String,
    pub event_id: Option<u64>
}

pub fn construct_match_request_plan(dialogue_data: DialogueData, index: u64) -> Result<DialoguePlan<'static>, String> {

    if let CaseData::MatchRequest(_) = dialogue_data.data {

    } else {
        return Err(format!("provided dialogue data does not fit to constructer function"))
    }

    let res = DialoguePlan {
        index,
        dialogue_data,
        steps: vec![
            DialogueStep {  //0 Someone requested a match with you
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    let match_info: MatchEvent = match dialogue_data.data.clone() {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data.match_info,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let parsed_initiator_id = match match_info.initiator_id.parse() {
                        Err(e) => return Err("initiator id couldnt be parsed in dialogue route".to_string()),
                        Ok(v) => v
                    };
                    let challenger: String = match UserId::new(parsed_initiator_id).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    let timestamp = match_info.start_timestamp;
                    Ok(format!("**{challenger} has requested a match with you on <t:{timestamp}:F>**. 
You can accept his proposal via reacting with {ACCEPT_EMOJI} or decline with {DECLINE_EMOJI}."))
                })))), 
                condition: StepCondition::React(Arc::new(Mutex::new(Box::new(|reaction: Option<bool>, dialogue_data: &mut DialogueData, app_state: &AppState| Box::pin(async move {

                    let info: &mut MatchRequestData = match &mut dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let match_info: MatchEvent = info.match_info.clone();
                    let match_id: String = match_info.get_id();
                    
                    match reaction {
                        None => {
                            let matches_clone = app_state.matchevents.clone();
                            let matches_lock = matches_clone.lock().await;

                            match matches_lock.get(&match_id) {
                                Some(entry) => {
                                    match entry.status {
                                        MatchStatus::Requested => return Ok(None),
                                        MatchStatus::Confirmed => {
                                            let event = create_discord_event(match_info, info.division_name.clone()).await?;
                                            info.event_id = Some(event.id.get());
                                            return Ok(Some(4))}, // confirmation via website was registered
                                        MatchStatus::Finished => return Ok(Some(6)), // match is already finished. status has been updated
                                        MatchStatus::Declined => return Ok(Some(5)), // decline via website was registered
                                    }
                                },
                                None => return Ok(Some(3)), // internal error: match couldnt be found in databank
                            }

                        },
                        Some(confirmation) =>{
                            let matches_clone = app_state.matchevents.clone();
                            let mut matches_lock = matches_clone.lock().await;

                            match matches_lock.get_mut(&match_id) {
                                Some(entry) => {
                                    if confirmation {
                                        entry.status = MatchStatus::Confirmed;
                                        let event = create_discord_event(match_info, info.division_name.clone()).await?;
                                        info.event_id = Some(event.id.get());
                                        return Ok(Some(1)) // confirmation has been registered
                                    }
                                    else {
                                        entry.status = MatchStatus::Declined;
                                        return Ok(Some(2)) // decline has been registered
                                    }
                                },
                                None => return Ok(Some(3)), // internal error: match couldnt be found in databank
                            }
                        },
                    }
                })))))
            },
            DialogueStep {  //1 your match was registered successfully
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    let info: &MatchRequestData = match &dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_initiator_id = match match_info.initiator_id.parse() {
                        Err(err) => return Err(format!("initiator id couldnt be parsed in dialogue route with error: {err}")),
                        Ok(v) => v
                    };
                    let challenger_tag: String = match UserId::new(parsed_initiator_id).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    let event_link = format!("https://discord.com/events/{}/{}", SERVER_ID, info.event_id.unwrap_or_else(|| 0));
                    Ok(format!("Your match against {challenger_tag} has been registered successfully. An event has been created on the PORC discord server: 
{event_link}"))
                })))), 
                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|_dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                    Ok(Some(600))
                })))))
            },
            DialogueStep {  //2 your decline was registered
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    let info: &MatchRequestData = match &dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_initiator_id = match match_info.initiator_id.parse() {
                        Err(err) => return Err(format!("initiator id couldnt be parsed in dialogue route with error: {err}")),
                        Ok(v) => v
                    };
                    let challenger_tag: String = match UserId::new(parsed_initiator_id).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    Ok(format!("Your decline against {challenger_tag} has been registered successfully."))
                })))), 
                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|_dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                    Ok(Some(600))
                })))))
            },
            DialogueStep {  //3 internal error: couldnt find match in databank
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    Ok(format!("Looks like I cant find your match in the databank :(. Feel free to contact the PORC mods about this"))
                })))), 
                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|_dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                    Ok(Some(600))
                })))))
            },
            DialogueStep {  //4 your confirmation via porcweb was registered
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    let info: &MatchRequestData = match &dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_initiator_id = match match_info.initiator_id.parse() {
                        Err(err) => return Err(format!("initiator id couldnt be parsed in dialogue route with error: {err}")),
                        Ok(v) => v
                    };
                    let challenger_tag: String = match UserId::new(parsed_initiator_id).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    Ok(format!("Your confirmation of your match against **{challenger_tag}** via the website has been registered successfully."))
                })))), 
                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|_dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                    Ok(Some(600))
                })))))
            },
            DialogueStep {  //5 your decline via porcweb was registered
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    let info: &MatchRequestData = match &dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_initiator_id = match match_info.initiator_id.parse() {
                        Err(err) => return Err(format!("initiator id couldnt be parsed in dialogue route with error: {err}")),
                        Ok(v) => v
                    };
                    let challenger_tag: String = match UserId::new(parsed_initiator_id).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    Ok(format!("Your decline of your match against **{challenger_tag}** via the website has been registered successfully."))
                })))), 
                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|_dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                    Ok(Some(600))
                })))))
            },
            DialogueStep {  //6 match already finished
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    let info: &MatchRequestData = match &dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_initiator_id = match match_info.initiator_id.parse() {
                        Err(err) => return Err(format!("initiator id couldnt be parsed in dialogue route with error: {err}")),
                        Ok(v) => v
                    };
                    let challenger_tag: String = match UserId::new(parsed_initiator_id).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    Ok(format!("Oh, looks like your match against **{challenger_tag}** is already finished. Guess Ill just count that as you confirming their offer"))
                })))), 
                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|_dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                    Ok(Some(600))
                })))))
            },
        ],
    };

    return Ok(res)
}