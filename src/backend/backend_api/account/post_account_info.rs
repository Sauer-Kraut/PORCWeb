use futures::join;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{backend::backend_api::server_error::ServerError, liberary::account_lib::{account::{pub_account_info::PubAccountInfo, storage::store_pub_account::store_pub_account}, availability::storage::update_availabilities::update_availabilities, login::storage::get_login::get_login}, AppState};



#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    auth_key: String,
    account_info: PubAccountInfo
}

// POST Request to store account info
// This wont alter match events
pub async fn post_account_info_request(info: web::Json<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let login = get_login(info.auth_key.clone(), appstate.pool.clone()).await?;

    if login.account_id != info.account_info.id {
        return Err(ServerError::Unauthorized)
    } 
    else if info.account_info.schedule.is_none() {
        return Err(ServerError::BadInput("Schedule cant be null".to_string()))
    }

    let account_fut = store_pub_account(info.account_info.clone(), appstate.pool.clone());
    let avail_fut = update_availabilities(info.account_info.id.clone(), info.account_info.schedule.clone().unwrap().availabilities, appstate.pool.clone());

    let (account_res, avail_res) = join!(account_fut, avail_fut);

    account_res?;
    avail_res?;

    Ok(HttpResponse::Ok())
}