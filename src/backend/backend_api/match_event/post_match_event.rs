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
            if v.account_id != info.match_event.challenger_id {
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

            let mut league = "".to_string();
            for division in matchplan.divisions.iter() {
                for player in division.players.iter() {
                    if (player.id == info.match_event.challenger_id ||
                        player.id == info.match_event.opponent_id) {
                        league = division.name.clone();
                    }
                }
            }
                        
            let _ =  make_bot_request_match(info.match_event.clone(), league, &state_clone).await?;
        }
    }

    Ok(HttpResponse::Ok())
}



pub async fn make_bot_request_match(matchevent: MatchEvent, league: String, appstate: &AppState) -> Result<(), String>{
    let parsed_opponent_id = matchevent.opponent_id.clone();

    let builder = DialogueInitator::initiate_match_request(parsed_opponent_id, league, matchevent).await?;

    // TODO!!!!! THIS ONE IS IMPORTANT!!!!
    // is this done? probably by now, I dont think it would work otherwise. But Im going to let this comment stay for now
    let res = store_dialogue(builder, appstate.pool.clone()).await;
    match res {
        Ok(_) => {},
        Err(err) => {
            return Err(format!("Error while storing dialogue: {:?}", err));
        }
    };
    Ok(())
}