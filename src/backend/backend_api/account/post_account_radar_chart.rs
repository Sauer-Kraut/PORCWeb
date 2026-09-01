use std::todo;

use futures::join;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{AppState, backend::backend_api::server_error::ServerError, liberary::account_lib::{account::{account_customisation::AccountCustomisation, storage::{store_account_cust::store_account_customisation, store_pub_account::store_pub_account, store_radar_chart::store_radar_chart}, user_data::radar_chart::RadarChart}, availability::storage::update_availabilities::update_availabilities, login::storage::get_login::get_login}};



#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    auth_key: String,
    radar_chart: RadarChart
}

// POST Request to store account info
// This wont alter match events
pub async fn post_account_radar_chart_request(info: web::Json<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let login = get_login(info.auth_key.clone(), appstate.pool.clone()).await?;

    let _account_fut = store_radar_chart(login.account_id, info.radar_chart.clone(), appstate.pool.clone()).await?;

    Ok(HttpResponse::Ok())
}