use actix_web::{web, HttpResponse, Responder, cookie::Cookie};
use colored::Colorize;
use tokio;
use std::sync::{mpsc, Arc};
use serde::{Deserialize, Serialize};
use super::bot_communication::make_bot_request_match;

use crate::liberary::account_lib::account::pub_account_info::PubAccountInfo;
use crate::liberary::account_lib::account::storage::get_account::get_account;
use crate::liberary::account_lib::account::storage::get_account_full::get_account_full;
use crate::liberary::account_lib::availability::storage::update_availabilities::update_availabilities;
use crate::liberary::account_lib::match_event::match_event::MatchEvent;
use crate::liberary::account_lib::match_event::storage::get_match_event::get_match_event;
use crate::liberary::account_lib::match_event::storage::get_match_events_from_ids::get_match_events_from_ids;
use crate::liberary::account_lib::schedule::schedule::Schedule;
use crate::liberary::matchplan_lib::matchplan::storage::matchplan_get::get_matchplan;
use crate::AppState;
use crate::liberary::account_lib::account::storage::store_pub_account::*;
use crate::liberary::account_lib::match_event::storage::store_match_event::*;



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


// Request to get accounts with the given ids
pub async fn put_account_info(info: web::Json<PutAccountInfoRecv>, appstate: web::Data<AppState>) -> impl Responder {

    let result: Result<Vec<PubAccountInfo>, String> = 'scope: {

        let mut account_infos = vec!();
        
        for user_id in info.ids.clone() {

            match get_account_full(user_id, appstate.pool.clone()).await {
                Ok(value) => {account_infos.push(value.get_pub_info())}
                Err(e) => {} // ignoring error, since finding nothing also returns an error
            }
        }

        if account_infos.is_empty() {
            break 'scope Err(format!("No accounts for the ids found"));
        } else {
            break 'scope Ok(account_infos);
        }
    };

    match result {
        Ok(data) => {
            return HttpResponse::Ok().json(PutAccountInfoResp {
                title: "Account Info response".to_string(),
                data: Some(data),
                error: None
            })
        },
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.red().bold());
            return HttpResponse::Ok().json(PutAccountInfoResp {
                title: "Account Info response".to_string(),
                data: None,
                error: Some(err)
            })
        }
    }
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

// Request to store a match event
// if match event is a new request a match request dialgoue will also be started
pub async fn post_match_event(info: web::Json<PostMatchEventRecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, Box<dyn std::error::Error>> {

    let state_clone = appstate.clone();

    let result: Result<(), String> = 'scope: {

        let match_event_entry = get_match_event(info.match_event.challenger_id.clone(), info.match_event.opponent_id.clone(), info.match_event.start_timestamp, info.match_event.season.clone(), appstate.pool.clone()).await;
            
        match match_event_entry {
            Ok(_entry) => {
                match store_match_event(info.match_event.clone(), appstate.pool.clone()).await {
                    Ok(_) => {
                        break 'scope Ok(());
                    },
                    Err(err) => {
                        break 'scope Err(format!("Error while storing match event: {:?}", err));
                    }
                };
            },
            Err(_) => {
                match store_match_event(info.match_event.clone(), appstate.pool.clone()).await {
                    Ok(_) => {},
                    Err(err) => {
                        break 'scope Err(format!("Error while storing match event: {:?}", err));
                    }
                };


                match get_matchplan(info.match_event.season.clone(), appstate.pool.clone()).await {
                    Ok(matchplan) => {
                        let mut league = "".to_string();
                        for division in matchplan.divisions.iter() {
                            for player in division.players.iter() {
                                if (player.id == info.match_event.challenger_id ||
                                    player.id == info.match_event.opponent_id) {
                                        league = division.name.clone();
                                    }
                            }
                        }

                        match make_bot_request_match(info.match_event.clone(), league, &state_clone).await {
                            Ok(_) => {
                                break 'scope Ok(());
                            },
                            Err(err) => {
                                break 'scope Err(format!("Got the following error while trying to communitcate with porcbot: {:?}", err)
                            )}
                        }
                    }
                    Err(err) => {
                        break 'scope Err(format!("Error while getting matchplan: {:?}", err));
                    }
                };
            }
        }
    };

    match result {
        Ok(_) => {
            return Ok(HttpResponse::Ok().json(PostMatchEventRespPackage {
                title: "Server Match Event update Respons".to_string(),
                error: None
            }))
        },
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.red().bold());
            return Ok(HttpResponse::Ok().json(PostMatchEventRespPackage {
                title: "Server Match Event update Respons".to_string(),
                error: Some(err)
            }))
        }
    }
}





#[derive(Serialize, Deserialize, Debug)]
pub struct PutMatchEventRecvPackage {
    pub title: String,
    match_events: Vec<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutMatchEventRespPackage {
    pub title: String,
    pub data: Vec<MatchEvent>,
    pub error: Option<String>
}

// Request to get match events from match ids
pub async fn put_match_event(info: web::Json<PutMatchEventRecvPackage>, appstate: web::Data<AppState>) -> impl Responder {

    let mut error = None;
    let mut match_events = vec!();

    // for event in info.match_events.iter() {
    //     let match_event_entry = get_match_event(event.challenger_id, event.opponent_id, event.timestamp, event.season.clone(), appstate.pool.clone()).await;
        
    //     match match_event_entry {
    //         Ok(entry) => {
    //             match_events.push(entry);
    //         },
    //         Err(_) => {
    //             error = Some(format!("No match event found for the match info: {:?}", event));
    //         }
    //     };
    // }

    let events = get_match_events_from_ids(info.match_events.clone(), appstate.pool.clone()).await;
    match events {
        Ok(events) => {
            match_events = events;
        },
        Err(err) => {
            error = Some(format!("Error while getting match events: {:?}", err));
        }
    };

    HttpResponse::Ok().json(PutMatchEventRespPackage {
        title: "Server GET match events Respons".to_string(),
        data: match_events,
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

// Request to store account info
// This wont alter match events
pub async fn post_account_info(info: web::Json<PostAccountInfoRecvPackage>, appstate: web::Data<AppState>) -> impl Responder {

    let mut error = None;

    match store_pub_account(info.account_info.clone(), appstate.pool.clone()).await {
        Ok(_) => {},
        Err(err) => {
            error = Some(format!("Error while storing account info: {:?}", err));
        }
    };

    match update_availabilities(info.account_info.id.clone(), info.account_info.schedule.clone().unwrap_or(Schedule { availabilities: vec![], matches: vec![], note: "".to_owned()}).availabilities, appstate.pool.clone()).await {
        Ok(_) => {},
        Err(err) => {
            error = Some(format!("Error while updating availabilities: {:?}", err));
        }
    };


    HttpResponse::Ok().json(PostAccountInfoRespPackage {
        title: "Server Logged in response".to_string(),
        error
    })
}