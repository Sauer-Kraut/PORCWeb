use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::liberary::account_lib::account::pub_account_info::PubAccountInfo;
use crate::liberary::account_lib::account::storage::get_account::get_account;
use crate::liberary::account_lib::login::storage::get_login::get_login;
use crate::AppState;
use crate::liberary::discord_lib::discord_event::discord_event::DiscordEvent;




#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespPackage {
    pub discord_events: Vec<DiscordEvent>,
}



// GET Request to receive all scheduled discord events
pub async fn get_discord_events_reqeust(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let events = appstate.discord_events.read().await.clone();

    Ok(HttpResponse::Ok().json(RespPackage {
        discord_events: events,
    }))
}