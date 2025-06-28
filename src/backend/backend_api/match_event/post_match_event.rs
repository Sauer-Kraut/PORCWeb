use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};
use futures::join;
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::liberary::account_lib::login::storage::get_login::get_login;
use crate::liberary::account_lib::match_event::match_event::MatchEvent;
use crate::liberary::account_lib::match_event::storage::get_match_event::get_match_event;
use crate::liberary::account_lib::match_event::storage::store_match_event::store_match_event;
use crate::liberary::dialogue_lib::dialogue_builder::storage::store_dialogue::store_dialogue;
use crate::liberary::dialogue_lib::dialogue_initiator::dialogue_initiator::DialogueInitator;
use crate::liberary::matchplan_lib::division::division::Division;
use crate::liberary::matchplan_lib::matchplan::storage::matchplan_get::get_matchplan;
use crate::AppState;



#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    pub match_event: MatchEvent,
    pub auth_key: String
}

// POST Request to store a match event
// if match event is a new request a match request dialgoue will also be started
pub async fn post_match_event_request(info: web::Json<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let state_clone = appstate.clone();

    let login_fut = get_login(info.auth_key.clone(), appstate.pool.clone());
    let match_event_entry_fut = get_match_event(info.match_event.challenger_id.clone(), info.match_event.opponent_id.clone(), info.match_event.start_timestamp, info.match_event.season.clone(), appstate.pool.clone());

    let (login_res, match_event_entry_res) = join!(login_fut, match_event_entry_fut);

    match login_res {
        Ok(v) => {
            if v.account_id != info.match_event.challenger_id || v.account_id != info.match_event.opponent_id {
                return Err(ServerError::Unauthorized)
            }
        },
        Err(_) => {
            return Err(ServerError::Unauthorized)
        },
    };
    
    let match_event_entry = match_event_entry_res?;
          
    match match_event_entry {
        Some(_entry) => {
            let _ = store_match_event(info.match_event.clone(), appstate.pool.clone()).await?;
        },
        None => {

            let store_fut = store_match_event(info.match_event.clone(), appstate.pool.clone());

            let season = match appstate.season.read().await.clone() {
                Some(season) => season.name.clone(),
                None => {
                    return Err(ServerError::Other("No current season found".to_owned().into()));
                }
            };

            let matchplan_fut = get_matchplan(season.clone(), appstate.pool.clone());

            let (store_res, matchplan_res) = join!(store_fut, matchplan_fut);

            store_res?;
            let matchplan = matchplan_res?;

            let league = matchplan.divisions.iter().find(|d| d.players.iter().any(|p| p.id == info.match_event.challenger_id || p.id == info.match_event.opponent_id))
                .unwrap_or(&Division {
                    name: "Unknown".to_string(),
                    order: 0,
                    matches: HashMap::new(),
                    players: vec![],
                }).name.clone();
                        
            let _ = DialogueInitator::initiate_match_request(&appstate, info.match_event.opponent_id.clone(), league, info.match_event.clone()).await?;
        }
    }

    Ok(HttpResponse::Ok())
}