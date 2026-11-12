use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::backend::backend_api::util::parse_query_param::parse_query_param_to_vec;
use crate::liberary::account_lib::account::pub_account_info::PubAccountInfo;
use crate::liberary::account_lib::account::storage::get_account_full::get_accounts_full;
use crate::liberary::account_lib::match_event::match_event::MatchEvent;
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

    let mut event_map = HashMap::new();
    let mut account_infos = vec!();
        
    for (value, match_events) in get_accounts_full(query_ids, appstate.pool.clone()).await? {
        for match_event in match_events {
            if let Some(id) = match_event.id {
                let _ = event_map.insert(id, match_event);
            }
        }

        account_infos.push(value.get_pub_info())
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