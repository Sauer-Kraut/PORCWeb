use futures::join;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{AppState, backend::backend_api::server_error::ServerError, liberary::account_lib::{account::{account_customisation::AccountCustomisation, storage::{store_account_cust::store_account_customisation, store_pose_stats::store_pose_stats, store_pub_account::store_pub_account}}, availability::storage::update_availabilities::update_availabilities, login::storage::get_login::get_login}};



#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    auth_key: String,
    account_cust: AccountCustomisation,
    pose_stats: Option<Vec<(String, f32)>>,
}

// POST Request to store account info
// This wont alter match events
pub async fn post_account_customisation_request(info: web::Json<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let login = get_login(info.auth_key.clone(), appstate.pool.clone()).await?;

    let _account_fut = store_account_customisation(login.account_id.clone(), info.account_cust.clone(), appstate.pool.clone()).await?;
    if let Some(stats) = info.pose_stats.clone() {
        let _pose_stats_fut = store_pose_stats(login.account_id.clone(), Some(stats), appstate.pool.clone()).await?;
    }

    Ok(HttpResponse::Ok())
}