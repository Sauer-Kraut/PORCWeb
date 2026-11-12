use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::backend::backend_api::util::parse_query_param::parse_query_param_to_vec;
use crate::liberary::account_lib::account::pub_account_info::PubAccountInfo;
use crate::liberary::account_lib::account::storage::get_account::get_accounts;
use crate::AppState;




#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    // limitation of the rust crate used to encode URLs, has to be parsed manually
    pub ids: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespPackage {
    pub accounts: Vec<PubAccountInfo>,
}


// GET request to get account Info without schedule
pub async fn get_account_info_request(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let query_ids = parse_query_param_to_vec::<String>(query.ids.clone())?;

    let account_infos = get_accounts(query_ids, appstate.pool.clone())
        .await?
        .into_iter()
        .map(|account| account.get_pub_info())
        .collect::<Vec<_>>();

    if account_infos.is_empty() {
        Err(sqlx::Error::RowNotFound.into())
    } else {
        Ok(HttpResponse::Ok().json(RespPackage {
            accounts: account_infos,
        }))
    }
}