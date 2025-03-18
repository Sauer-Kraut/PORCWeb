use actix_http::cookie;
use actix_web::cookie::time::error;
use async_std::sync::Mutex;
use actix_web::{FromRequest, HttpRequest};
use actix_web::{web, HttpResponse, Responder, cookie::Cookie};
use colored::Colorize;
use tokio;
use std::collections::HashMap;
use std::default;
use std::sync::{mpsc, Arc};
use serde::{Deserialize, Serialize};
use super::bot_communication::make_bot_request_match;
use super::data_lib::{Division, MatchPlan, Player};
use super::client_communication::{SignUpInfo, GetRequestPlanPackage, GetRequestSignUpPackage};
use super::storage_lib::StorageMod;
use crate::AppState;
use async_std::fs;
use reqwest::Client;
use uuid::Uuid;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Account {
    pub user_info: DiscordUser,
    pub schedule: Option<Schedule>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub email: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub availabilities: Vec<Event>,
    pub matches: Vec<String>,
    pub notes: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Event {
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub repetition: Repetition,
    pub repetition_config: Option<DailyRepetitionConfig>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Repetition {
    Once,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DailyRepetitionConfig {
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MatchEvent {
    pub start_timestamp: u64,
    pub initiator_id: String,
    pub opponent_id: String,
    pub status: MatchStatus
}

impl MatchEvent{

    pub fn get_id(&self) -> String {
        let mut lower_id = self.initiator_id.clone();
        let mut higher_id = self.opponent_id.clone();

        if lower_id.parse::<i64>().unwrap() > higher_id.parse::<i64>().unwrap() {
            lower_id = self.opponent_id.clone();
            higher_id = self.initiator_id.clone();
        }

        format!("{}V{}@{}", lower_id, higher_id, self.start_timestamp)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MatchStatus {
    Requested,
    Confirmed,
    Finished,
    Declined
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PubAccountInfo {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub schedule: Option<Schedule>
}

impl Account {

    pub fn get_pub_info(&self) -> PubAccountInfo {
        PubAccountInfo {
            id: self.user_info.id.clone(),
            username: self.user_info.username.clone(),
            avatar:  self.user_info.avatar.clone(),
            schedule: self.schedule.clone(),
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct PutAccountInfoRecv {
    pub title: String,
    pub ids: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutAccountInfoResp {
    pub title: String,
    pub data: Option<Vec<PubAccountInfo>>,
    pub error: Option<String>
}



pub async fn put_account_info(info: web::Json<PutAccountInfoRecv>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received PUT Request for account infos".bold().cyan());

    let (data_sender, data_receiver) = mpsc::channel();
    let (error_sender, error_receiver) = mpsc::channel();

    let accounts_clone = appstate.accounts.clone();
    let accounts_lock = accounts_clone.lock().await.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let mut account_infos = vec!();
        
        for user_id in info.ids.clone() {
            match accounts_lock.get(&user_id) {
                Some(value) => {account_infos.push(value.get_pub_info())}
                None => {}
            }
        }

        if account_infos.is_empty() {
            error_sender.send(format!("No accounts for the ids found")).unwrap()
        } else {
            data_sender.send(account_infos).unwrap()
        }
    }).await.unwrap();

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    let data = match data_receiver.recv() {
        Ok(data) => Some(data),
        Err(_) => None,
    };

    HttpResponse::Ok().json(PutAccountInfoResp {
        title: "Account Info response".to_string(),
        data,
        error
    })
}





#[derive(Serialize, Deserialize, Debug)]
pub struct PostMatchEventRecvPackage {
    pub title: String,
    pub match_event: MatchEvent
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMatchEventRespPackage {
    pub title: String,
    pub error: Option<String>
}

pub async fn post_match_event(info: web::Json<PostMatchEventRecvPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received POST Request for a match event".bold().cyan());

    let (error_sender, error_receiver) = mpsc::channel();
    
    let accounts_clone = appstate.accounts.clone();
    let matchevents_clone = appstate.matchevents.clone();
    let matchplan_clone = appstate.matchplan.clone();

    let state_clone = appstate.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let mut error = "".to_string();

        let mut accounts_lock = accounts_clone.lock().await;
        let mut accounts = accounts_lock.clone();

        let mut matchevents_lock = matchevents_clone.lock().await;
        let mut matchevents = matchevents_lock.clone();

        let mut matchplan_lock = matchplan_clone.lock().await;
        let mut matchplan = matchplan_lock.clone();
        
        let initiator = accounts.get_mut(&info.match_event.initiator_id);

        let match_key = info.match_event.get_id();

        match matchevents.get_mut(&match_key) {
            Some(matchevent) => {
                matchevent.status = info.match_event.status.clone();
            },
            None => {
                let matchevent = MatchEvent{
                    start_timestamp: info.match_event.start_timestamp,
                    initiator_id: info.match_event.initiator_id.clone(),
                    opponent_id: info.match_event.opponent_id.clone(),
                    status: MatchStatus::Requested,
                };
                let mut league = "".to_string();
                for division in matchplan.unwrap().divisions.iter() {
                    for player in division.players.iter() {
                        if (player.id == info.match_event.initiator_id ||
                            player.id == info.match_event.opponent_id) {
                                league = division.name.clone();
                            }
                    }
                }
                match make_bot_request_match(matchevent.clone(), league, &state_clone).await {
                    Ok(_) => {},
                    Err(err) => {println!("Got the following error while trying to communitcate with porcbot: {:?} fuck it we ball", err)}
                }
                matchevents.insert(match_key.clone(), matchevent);
            },
        }
        
        match initiator {
            Some(account) => {

                match account.schedule {
                    None => {
                        account.schedule = Some(Schedule {
                            availabilities: vec!(),
                            matches: vec!(),
                            notes: "".to_string(),
                        });
                    },
                    _ => {}
                }

                let mut schedule_copie = account.schedule.clone().unwrap();

                let mut found = false;
                for match_event_key in schedule_copie.matches.iter_mut() {
                    if *match_event_key == match_key {
                        found = true
                    }
                }

                if !found {
                    schedule_copie.matches.push(match_key.clone());
                }

                account.schedule = Some(schedule_copie);
            },
            None => {error = "Not all participating accounts could be found".to_string()},
        };

        
        let opponent = accounts.get_mut(&info.match_event.opponent_id);
        
        match opponent {
            Some(account) => {

                match account.schedule {
                    None => {
                        account.schedule = Some(Schedule {
                            availabilities: vec!(),
                            matches: vec!(),
                            notes: "".to_string(),
                        });
                    },
                    _ => {}
                }

                let mut schedule_copie = account.schedule.clone().unwrap();

                let mut found = false;
                for match_event_key in schedule_copie.matches.iter_mut() {
                    if *match_event_key == match_key {
                        found = true
                    }
                }

                if !found {
                    schedule_copie.matches.push(match_key);
                }

                account.schedule = Some(schedule_copie);
            },
            None => {error = "Not all participating accounts could be found".to_string()},
        };

        if error == "".to_string() {
            *accounts_lock = accounts;
            *matchevents_lock = matchevents;
        } else {
            error_sender.send(error).unwrap();
        }
    }).await.unwrap();

    let _ = appstate.refresh().await;

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    HttpResponse::Ok().json(PostMatchEventRespPackage {
        title: "Server Match Event update Respons".to_string(),
        error
    })
}





#[derive(Serialize, Deserialize, Debug)]
pub struct PutMatchEventRecvPackage {
    pub title: String,
    pub match_events: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutMatchEventRespPackage {
    pub title: String,
    pub data: Vec<MatchEvent>,
    pub error: Option<String>
}

pub async fn put_match_event(info: web::Json<PutMatchEventRecvPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received PUT Request for match events".bold().cyan());

    let (data_sender, data_receiver) = mpsc::channel();
    let (error_sender, error_receiver) = mpsc::channel();
    let matchevents_clone = appstate.matchevents.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let matchevents_lock = matchevents_clone.lock().await;
        let matchevents = matchevents_lock.clone();

        let mut found_events = vec!();

        for event_key in info.match_events.iter() {

            match matchevents.get(event_key){
                Some(event) => {found_events.push(event.clone())},
                None => {error_sender.send(format!("match event with key {} could not be found", event_key)).unwrap()},
            }
        }

        data_sender.send(found_events).unwrap_or_else( |err| {
            error_sender.send(format!("Internal Server Error : {:?}", err)).unwrap();
        });

    }).await.unwrap();

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    let data = data_receiver.recv().unwrap();

    HttpResponse::Ok().json(PutMatchEventRespPackage {
        title: "Server GET match events Respons".to_string(),
        data,
        error
    })
}







#[derive(Serialize, Deserialize, Debug)]
pub struct PostAccountInfoRecvPackage {
    title: String,
    client_id: String,
    account_info: PubAccountInfo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostAccountInfoRespPackage {
    pub title: String,
    pub error: Option<String>
}

// This cant alter matches, beacause these are handled via shared ids. Will simply ignore the matches field
pub async fn post_account_info(info: web::Json<PostAccountInfoRecvPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{} {}", "Received POST Request for account info of account:".bold().cyan(), info.account_info.username.bold().italic());

    let (error_sender, error_receiver) = mpsc::channel();

    let logins_mutex = appstate.logins.clone();
    let logins_lock = logins_mutex.lock().await;
    let logins_clone = logins_lock.clone();
    drop(logins_lock);

    let client_id = info.client_id.clone();

    let accounts_clone = appstate.accounts.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let mut accounts_lock = accounts_clone.lock().await;
        let mut accounts = accounts_lock.clone();

        let mut error = "".to_string();
        
        match logins_clone.get(&client_id) {
            Some(entry) => {
                let discord_id = entry.clone();
                match accounts.get_mut(&discord_id) {
                    Some(value) => {
                        let default_schedule = Schedule {
                            availabilities: vec!(),
                            matches: vec!(),
                            notes: "".to_string(),
                        };
                        println!("old availabilities: {:?}, new availabilities: {:?}",  
                            value.schedule.clone().unwrap_or(default_schedule.clone()).availabilities,
                            info.account_info.schedule.clone().unwrap_or(default_schedule.clone()).availabilities);
                            
                        *value = Account{ 
                            user_info: DiscordUser {
                                id: info.account_info.id.clone(),
                                username: info.account_info.username.clone(),
                                discriminator: value.user_info.discriminator.clone(),
                                avatar: info.account_info.avatar.clone(),
                                email: value.user_info.email.clone(),
                            }, 
                            schedule: Some(Schedule {
                                availabilities: match info.account_info.schedule.clone() {
                                    Some(schedule) => schedule.availabilities,
                                    None => value.schedule.clone().unwrap_or(default_schedule.clone()).availabilities
                                },
                                matches: value.schedule.clone().unwrap_or(default_schedule.clone()).matches,
                                notes: info.account_info.schedule.clone().unwrap_or(default_schedule.clone()).notes,
                            })
                        }}
                    None => error = "No account for discord id found".to_string()
                }
            },
            None => error = "no login found for client id".to_string()
        };

        if error == "".to_string() {
            *accounts_lock = accounts;
        } else {
            error_sender.send(error).unwrap();
        }

        drop(accounts_lock);

    }).await.unwrap();

    let _ = appstate.refresh().await;

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    HttpResponse::Ok().json(PostAccountInfoRespPackage {
        title: "Server Logged in response".to_string(),
        error
    })
}