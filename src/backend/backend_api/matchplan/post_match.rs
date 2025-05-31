use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::liberary::account_lib::login::storage::get_login::get_login;
use crate::liberary::matchplan_lib::matchplan_match::storage::match_store::update_match;
use crate::{liberary::matchplan_lib::matchplan_match::matchplan_match::Match};
use crate::AppState;



#[derive(Debug, Deserialize, Serialize)]
pub struct RecvPackage {
    pub match_info: Match,
    pub auth_key: String
}


// POST Request to update a provided match of the current season
pub async fn post_match_request(info: web::Json<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let login = get_login(info.auth_key.clone(), appstate.pool.clone()).await?;

    if !(login.account_id == info.match_info.p1.id || login.account_id == info.match_info.p2.id) {
        return Err(ServerError::Unauthorized)
    }

    let season = appstate.get_season().await?.name;
            
    update_match(info.match_info.clone(), season, appstate.pool.clone()).await?;

    Ok(HttpResponse::Ok())
}