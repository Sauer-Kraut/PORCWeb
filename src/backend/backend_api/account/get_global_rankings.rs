use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::backend::backend_api::util::parse_query_param::parse_query_param_to_vec;
use crate::liberary::account_lib::account::pub_account_info::PubAccountInfo;
use crate::liberary::account_lib::account::storage::get_account_full::get_account_full;
use crate::liberary::account_lib::account::storage::get_global_leaderboard::get_global_leaderboard;
use crate::liberary::account_lib::schedule::schedule::Schedule;
use crate::AppState;



#[derive(Serialize, Deserialize, Debug)]
pub struct RespPackage {
    pub rankings: Vec<PubAccountInfo>
}


// GET request to get cumulated wieghted rankings across the last three seasons
pub async fn get_global_ranking_request(appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let res = get_global_leaderboard(appstate.pool.clone()).await?;
    Ok(HttpResponse::Ok().json(RespPackage {
        rankings: res.iter().map(|a| a.get_pub_info()).collect()
    }))
}