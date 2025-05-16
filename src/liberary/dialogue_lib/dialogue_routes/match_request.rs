use std::sync::Arc;
use colored::Colorize;

use actix_web::web;
use async_std::sync::Mutex;
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use serenity::{all::{User, UserId}, model::event};


use crate::{liberary::{account_lib::match_event::{match_event::{MatchEvent, MatchStatus}, storage::{get_match_event::get_match_event, store_match_event}}, dialogue_lib::dialogue_plan::{dialogue_data::{self, CaseData, DialogueData}, dialogue_plan::DialoguePlan, dialogue_step::{DialogueStep, StepCondition}}}, porcbot::{config::*, util::{create_event::create_discord_event, dm_send::send_dm}}, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchRequestData {
    pub match_info: MatchEvent,
    pub division_name: String,
    pub event_id: Option<u64>
}

pub fn construct_match_request_plan(dialogue_data: DialogueData, index: u64, dialogue_id: Option<i64>) -> Result<DialoguePlan<'static>, String> {

    if let CaseData::MatchRequest(_) = dialogue_data.data {

    } else {
        return Err(format!("provided dialogue data does not fit to constructer function"))
    }

    let res = DialoguePlan {
        dialogue_id,
        index,
        dialogue_data,
        steps: vec![
            DialogueStep {  //0 Someone requested a match with you
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    let match_info: MatchEvent = match dialogue_data.data.clone() {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data.match_info,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let parsed_challenger_id = match_info.challenger_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?;
                    let challenger: String = match UserId::new(parsed_challenger_id).to_user(get_http()).await {
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

                    let mut entry = match get_match_event(match_info.challenger_id.clone(), match_info.opponent_id.clone(), match_info.start_timestamp, match_info.season.clone(), app_state.pool.clone()).await {
                        Ok(v) => v,
                        Err(err) => {println!("{}", format!("{}{}", "An error occured while checking dialogue: ".red(), err.to_string().bright_red())); return Ok(Some(3))}, // internal error: match couldnt be found in databank
                    };
                    
                    match reaction {
                        None => {

                            match entry.status {
                                MatchStatus::Requested => return Ok(None),
                                MatchStatus::Confirmed => {
                                    let planed_event = create_discord_event(match_info.clone(), info.division_name.clone()).await?;
                                    let opponent_tag: String = match UserId::new(match_info.opponent_id.clone().parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await {
                                        Ok(v) => v.name,
                                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                                    };

                                    let event_link = format!("https://discord.com/events/{}/{}", **SERVER_ID, planed_event.id.get());
                                    let _ = send_dm(match_info.challenger_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?, format!("Your requested match with {opponent_tag} has been accepted:
{event_link}")).await;
                                    info.event_id = Some(planed_event.id.get());

                                    return Ok(Some(4)) // confirmation via website was registered
                                }, 
                                MatchStatus::Finished => return Ok(Some(6)), // match is already finished. status has been updated
                                MatchStatus::Declined => return Ok(Some(5)), // decline via website was registered
                            }
        
                        },
                        Some(confirmation) => {

                            if confirmation {
                                entry.status = MatchStatus::Confirmed;
                                match store_match_event::store_match_event(entry.clone(), app_state.pool.clone()).await {
                                    Ok(_) => {},
                                    Err(err) => return Err(format!("error while storing match event in databank: {err}")),
                                };

                                let planed_event = create_discord_event(match_info.clone(), info.division_name.clone()).await?;
                                info.event_id = Some(planed_event.id.get());

                                return Ok(Some(1)) // confirmation has been registered
                            }
                            else {
                                entry.status = MatchStatus::Declined;
                                match store_match_event::store_match_event(entry.clone(), app_state.pool.clone()).await {
                                    Ok(_) => {},
                                    Err(err) => return Err(format!("error while storing match event in databank: {err}")),
                                };

                                return Ok(Some(2)) // decline has been registered
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
                    let parsed_challenger_id = match_info.challenger_id;
                    let challenger_tag: String = match UserId::new(parsed_challenger_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    let event_link = format!("https://discord.com/events/{}/{}", **SERVER_ID, info.event_id.unwrap_or_else(|| 0));
                    Ok(format!("Your match against {challenger_tag} has been registered successfully. An event has been created on the PORC discord server: 
{event_link}"))
                })))), 
                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                    let info: &mut MatchRequestData = match &mut dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_opponent_id = match_info.opponent_id;
                    let opponent_tag: String = match UserId::new(parsed_opponent_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    let parsed_challenger_id = match_info.challenger_id;
                    let event_id = match info.event_id {
                        Some(id) => id,
                        None => return Err(format!("couldnt find event id for event that should be configured"))
                    };
                    let event_link = format!("https://discord.com/events/{}/{}", **SERVER_ID, event_id);
                    let timestamp = &info.match_info.start_timestamp;
                    let _ = send_dm(parsed_challenger_id, format!("Your requested match with {opponent_tag} at <t:{timestamp}:F> has been accepted:
{event_link}")).await;
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
                    let parsed_challenger_id = match_info.challenger_id;
                    let challenger_tag: String = match UserId::new(parsed_challenger_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    Ok(format!("Your decline against {challenger_tag} has been registered successfully."))
                })))), 
                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                    let info: &mut MatchRequestData = match &mut dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_opponent_id = match_info.opponent_id;
                    let opponent_tag: String = match UserId::new(parsed_opponent_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    let parsed_challenger_id = match_info.challenger_id;
                    let timestamp = &info.match_info.start_timestamp;
                    let _ = send_dm(parsed_challenger_id, format!("Your requested match with {opponent_tag} at <t:{timestamp}:F> has been declined")).await;
                    Ok(Some(600))
                })))))
            },
            DialogueStep {  //3 internal error: couldnt find match in databank
                message: Arc::new(Mutex::new(Box::new(|_dialogue_data: &DialogueData| Box::pin(async move {
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
                    let parsed_challenger_id = match_info.challenger_id;
                    let challenger_tag: String = match UserId::new(parsed_challenger_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    Ok(format!("Your confirmation of your match against **{challenger_tag}** via the website has been registered successfully."))
                })))), 
                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                    let info: &mut MatchRequestData = match &mut dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_opponent_id = match_info.opponent_id;
                    let opponent_tag: String = match UserId::new(parsed_opponent_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    let parsed_challenger_id = match_info.challenger_id;
                    let event_id = match info.event_id {
                        Some(id) => id,
                        None => return Err(format!("couldnt find event id for event that should be configured"))
                    };
                    let event_link = format!("https://discord.com/events/{}/{}", **SERVER_ID, event_id);
                    let timestamp = &info.match_info.start_timestamp;
                    let _ = send_dm(parsed_challenger_id, format!("Your requested match with {opponent_tag} at <t:{timestamp}:F> has been accepted:
{event_link}")).await;
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
                    let parsed_challenger_id = match_info.challenger_id;
                    let challenger_tag: String = match UserId::new(parsed_challenger_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    Ok(format!("Your decline of your match against **{challenger_tag}** via the website has been registered successfully."))
                })))), 
                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|dialogue_data: &mut DialogueData, _app_state: &AppState| Box::pin(async move {
                    let info: &mut MatchRequestData = match &mut dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => panic!("Dialogue Route has incorect Case data")
                    };
                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_opponent_id = match_info.opponent_id;
                    let opponent_tag: String = match UserId::new(parsed_opponent_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await {
                        Ok(v) => v.name,
                        Err(err) => return Err(format!("user id couldnt be converted to user in dialogue route with err: {err}"))
                    };
                    let parsed_challenger_id = match_info.challenger_id;
                    let timestamp = &info.match_info.start_timestamp;
                    let _ = send_dm(parsed_challenger_id, format!("Your requested match with {opponent_tag} at <t:{timestamp}:F> has been declined")).await;
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
                    let parsed_challenger_id = match_info.challenger_id;
                    let challenger_tag: String = match UserId::new(parsed_challenger_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await {
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