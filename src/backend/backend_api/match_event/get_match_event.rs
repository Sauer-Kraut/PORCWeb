use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::backend::backend_api::util::parse_query_param::parse_query_param_to_vec;
use crate::liberary::account_lib::match_event::match_event::MatchEvent;
use crate::liberary::account_lib::match_event::storage::get_match_events_from_ids::get_match_events_from_ids;
use crate::AppState;


#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    // Vec<i32> as String since sequence parameters are fucked
    pub match_events: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespPackage {
    pub match_events: Vec<MatchEvent>
}

// GET Request to get match events from match ids
pub async fn get_match_event_request(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let query_ids = parse_query_param_to_vec::<i32>(query.match_events.clone())?;

    let events = get_match_events_from_ids(query_ids, appstate.pool.clone()).await?;

    Ok(HttpResponse::Ok().json(RespPackage {
        match_events: events
    }))
}


