use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::backend::backend_api::util::parse_query_param::parse_query_param_to_vec;
use crate::liberary::account_lib::account::pub_account_info::PubAccountInfo;
use crate::liberary::account_lib::account::storage::get_account_full::get_account_full;
use crate::liberary::account_lib::match_event::match_event::MatchEvent;
use crate::liberary::account_lib::match_event::storage::get_match_events_from_ids::get_match_events_from_ids;
use crate::liberary::account_lib::schedule::schedule::Schedule;
use crate::AppState;


#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    // limitation of the rust crate used to encode URLs, has to be parsed manually
    pub ids: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespPackage {
    pub accounts: Vec<PubAccountInfo>,
    pub match_events: HashMap<i32, MatchEvent>
}


// GET request to get account Info with schedule and all match events
pub async fn get_account_info_full_request(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let query_ids = parse_query_param_to_vec::<String>(query.ids.clone())?;

    let mut account_info_handles = vec!();
    let mut account_infos = vec!();

    for user_id in query_ids {

        account_info_handles.push(get_account_full(user_id, appstate.pool.clone()));
    }

    let mut match_event_ids = vec!();
    let mut event_map = HashMap::new();
        
    for account_info in join_all(account_info_handles).await {

        match account_info {
            Ok((value, match_events)) => {

                for match_event in match_events.iter() {

                    let _ = event_map.insert(match_event.id.unwrap(), match_event.clone());
                }
                
                for id in value.clone().schedule.unwrap_or(Schedule{ availabilities: vec!(), matches: vec!(), note: "".to_string() }).matches.iter() {

                    if !match_event_ids.contains(id) {
                        match_event_ids.push(*id);
                    }
                }
            
                account_infos.push(value.get_pub_info())
            }
            Err(_e) => {} // ignoring error, since finding nothing also returns an error 
            // TODO: make better by making get_account option for that error specifically
        }
    }

    if account_infos.is_empty() {
        Err(sqlx::Error::RowNotFound.into())
    } else {
        Ok(HttpResponse::Ok().json(RespPackage {
            accounts: account_infos,
            match_events: event_map
        }))
    }
}