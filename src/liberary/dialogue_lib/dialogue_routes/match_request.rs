use std::sync::Arc;
use async_std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serenity::all::UserId;


use crate::{liberary::{account_lib::match_event::{match_event::{MatchEvent, MatchStatus}, storage::{get_match_event::get_match_event, store_match_event}}, dialogue_lib::{bot_error::BotError, dialogue_initiator::dialogue_initiator::DialogueInitator, dialogue_plan::{dialogue_data::{self, CaseData, DialogueData}, dialogue_plan::DialoguePlan, dialogue_step::{DialogueStep, StepCondition}}}, util::functions::parse_user_id::parse_ds_user_id}, porcbot::{config::*, util::{create_event::create_discord_event}}, AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchRequestData {
    pub match_info: MatchEvent,
    pub division_name: String,
    pub event_id: Option<u64>
}

// not perfectly efficient but makes the code a lot cleaner, plus clone sizes should be pretty small
impl TryFrom<DialogueData> for MatchRequestData {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(value: DialogueData) -> Result<Self, Self::Error> {
        if let CaseData::MatchRequest(match_request_data) = value.data {
            Ok(match_request_data)
        } else {
            Err("DialogueData does not contain MatchRequestData".into())
        }
    }
}


pub fn construct_match_request_plan(dialogue_data: DialogueData, index: u64, dialogue_id: Option<i64>) -> Result<DialoguePlan<'static>, BotError> {

    if let CaseData::MatchRequest(_) = dialogue_data.data {

    } else {
        return Err(format!("provided dialogue data does not fit to constructer function").into())
    }

    let res = DialoguePlan {
        dialogue_id,
        index,
        dialogue_data,
        steps: vec![
            DialogueStep {  //0 Someone requested a match with you

                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {

                    let info: MatchRequestData = dialogue_data.clone().try_into()?;
                    let match_info = info.match_info;

                    let parsed_challenger_id = parse_ds_user_id(&match_info.challenger_id)?;
                    let challenger: String = parsed_challenger_id.to_user(get_http()).await
                        .map_err(|err| format!("user id couldnt be converted to user in dialogue route with err: {err}"))?
                        .name;

                    let timestamp = match_info.start_timestamp;

                    Ok(format!("**{challenger} has requested a match with you on <t:{timestamp}:F>**. \nYou can accept his proposal via reacting with {ACCEPT_EMOJI} or decline with {DECLINE_EMOJI}."))
                })))), 

                condition: StepCondition::React(Arc::new(Mutex::new(Box::new(|reaction: Option<bool>, dialogue_data: &mut DialogueData, appstate: &AppState| Box::pin(async move {

                    let info: &mut MatchRequestData = match dialogue_data.data {
                        CaseData::MatchRequest(ref mut match_request_data) => match_request_data,
                        _ => return Err("Dialogue Route has incorrect Case data".to_string().into())
                    };

                    let match_info: MatchEvent = info.match_info.clone();

                    let mut entry = get_match_event(match_info.challenger_id.clone(), match_info.opponent_id.clone(), match_info.start_timestamp, match_info.season.clone(), appstate.pool.clone()).await?
                        .ok_or("couldnt find match event entry".to_string())?;
                    
                    match reaction {
                        None => {

                            match entry.status {
                                MatchStatus::Requested => return Ok(None),
                                MatchStatus::Confirmed => {
                                    let planed_event = create_discord_event(match_info.clone(), info.division_name.clone()).await?;
                                    let opponent_tag: String = UserId::new(match_info.opponent_id.clone().parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?).to_user(get_http()).await?.name;

                                    let event_link = format!("https://discord.com/events/{}/{}", **SERVER_ID, planed_event.id.get());

                                    // Fail can only be caused by DB error, therefore fails cause complete clousure fail
                                    let confirmation_message = format!("Your requested match with {opponent_tag} has been accepted:\n{event_link}");
                                    let parsed_challenger_id = match_info.challenger_id.parse().map_err(|err| format!("couldnt parse eventId {err:?}"))?;
                                    let _res = DialogueInitator::initiate_info(appstate, parsed_challenger_id, confirmation_message).await?;

                                    info.event_id = Some(planed_event.id.get());

                                    return Ok(Some(12)) // confirmation was registered
                                }, 
                                MatchStatus::Finished => return Ok(Some(3)), // match is already finished. status has been updated
                                MatchStatus::Declined => return Ok(Some(2)), // decline was registered
                            }
        
                        },

                        Some(confirmation) => {

                            if confirmation {
                                entry.status = MatchStatus::Confirmed;
                                store_match_event::store_match_event(entry.clone(), appstate.pool.clone()).await.map_err(|err| format!("error while storing match event in databank: {err}"))?;
                                let planed_event = create_discord_event(match_info.clone(), info.division_name.clone()).await?;
                                info.event_id = Some(planed_event.id.get());

                                return Ok(Some(1)) // confirmation has been registered
                            }
                            else {
                                entry.status = MatchStatus::Declined;
                                store_match_event::store_match_event(entry.clone(), appstate.pool.clone()).await.map_err(|err| format!("error while storing match event in databank: {err}"))?;

                                return Ok(Some(2)) // decline has been registered
                            }
                                
                        },
                    }
                })))))
            },


            // 1 your match was registered successfully
            DialogueStep {  
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {
                    
                    let info: MatchRequestData = dialogue_data.clone().try_into()?;
                    let match_info: MatchEvent = info.match_info;
                    
                    let parsed_challenger_id = match_info.challenger_id.parse().map_err(|err| format!("couldnt parse user Id {err:?}"))?;
                    let challenger_tag: String = UserId::new(parsed_challenger_id).to_user(get_http()).await
                        .map_err(|err| format!("user id couldnt be converted to user in dialogue route with err: {err}"))?
                        .name;

                    let event_link = format!("https://discord.com/events/{}/{}", **SERVER_ID, info.event_id.unwrap_or_else(|| 0));
                    Ok(format!("Your match against {challenger_tag} has been registered successfully. An event has been created on the PORC discord server: \n{event_link}"))
                })))),

                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|dialogue_data: &mut DialogueData, appstate: &AppState| Box::pin(async move {

                    let info: &mut MatchRequestData = match dialogue_data.data {
                        CaseData::MatchRequest(ref mut match_request_data) => match_request_data,
                        _ => return Err("Dialogue Route has incorrect Case data".to_string().into())
                    };

                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_opponent_id = parse_ds_user_id(&match_info.opponent_id)?;
                    let opponent_tag: String = parsed_opponent_id.to_user(get_http()).await
                        .map_err(|err| format!("user id couldnt be converted to user in dialogue route with err: {err}"))?
                        .name;
                    
                    let event_id = match info.event_id {
                        Some(id) => id,
                        None => return Err(format!("couldnt find event id for event that should be configured").into())
                    };

                    let event_link = format!("https://discord.com/events/{}/{}", **SERVER_ID, event_id);
                    let timestamp = &info.match_info.start_timestamp;
                    let confirmation_message = format!("Your requested match with {opponent_tag} at <t:{timestamp}:F> has been accepted: \n{event_link}");
                    let _res = DialogueInitator::initiate_info(appstate, match_info.challenger_id, confirmation_message).await?;

                    Ok(Some(600))
                })))))
            },


            // 2 your decline was registered
            DialogueStep {
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {

                    let info: &MatchRequestData = match &dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => return Err("Dialogue Route has incorect Case data".to_string().into())
                    };

                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_challenger_id = parse_ds_user_id(&match_info.challenger_id)?;
                    let challenger_tag: String = parsed_challenger_id.to_user(get_http()).await
                        .map_err(|err| format!("user id couldnt be converted to user in dialogue route with err: {err}"))?
                        .name;

                    Ok(format!("Your decline against {challenger_tag} has been registered successfully."))
                })))), 

                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|dialogue_data: &mut DialogueData, appstate: &AppState| Box::pin(async move {

                    let info: &mut MatchRequestData = match &mut dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => return Err("Dialogue Route has incorect Case data".to_string().into())
                    };
                    
                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_opponent_id = parse_ds_user_id(&match_info.opponent_id)?;
                    let opponent_tag: String = parsed_opponent_id.to_user(get_http()).await
                        .map_err(|err| format!("user id couldnt be converted to user in dialogue route with err: {err}"))?
                        .name;
                    
                    let timestamp = &info.match_info.start_timestamp;
                    let decline_message = format!("Your requested match with {opponent_tag} at <t:{timestamp}:F> has been declined");
                    let _res = DialogueInitator::initiate_info(appstate, match_info.challenger_id, decline_message).await?;
                    Ok(Some(600))
                })))))
            },
            

            //3 match already finished
            DialogueStep {  
                message: Arc::new(Mutex::new(Box::new(|dialogue_data: &DialogueData| Box::pin(async move {

                    let info: &MatchRequestData = match &dialogue_data.data {
                        dialogue_data::CaseData::MatchRequest(match_request_data) => match_request_data,
                        _ => return Err("Dialogue Route has incorect Case data".to_string().into())
                    };

                    let match_info: MatchEvent = info.match_info.clone();
                    let parsed_challenger_id = parse_ds_user_id(&match_info.challenger_id)?;
                    let challenger_tag: String = parsed_challenger_id.to_user(get_http()).await
                        .map_err(|err| format!("user id couldnt be converted to user in dialogue route with err: {err}"))?
                        .name;

                    Ok(format!("Oh, looks like your match against **{challenger_tag}** is already finished. Guess Ill just count that as you confirming their offer"))
                })))), 

                condition: StepCondition::Info(Arc::new(Mutex::new(Box::new(|_dialogue_data: &mut DialogueData, _appstate: &AppState| Box::pin(async move {
                    Ok(Some(600))
                })))))
            },
        ],
    };

    return Ok(res)
}