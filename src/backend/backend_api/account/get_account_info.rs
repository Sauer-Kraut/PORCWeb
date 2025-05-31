use actix_web::{web, HttpResponse, Responder};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::backend::backend_api::util::parse_query_param::parse_query_param_to_vec;
use crate::liberary::account_lib::account::pub_account_info::PubAccountInfo;
use crate::liberary::account_lib::account::storage::get_account::get_account;
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

    let mut account_info_handles = vec!();
    let mut account_infos = vec!();

    for user_id in query_ids.clone() {

        account_info_handles.push(get_account(user_id, appstate.pool.clone()));
    }
        
    for account_info in join_all(account_info_handles).await {

        match account_info {
            Ok(value) => {account_infos.push(value.get_pub_info())}
            Err(_e) => {} // ignoring error, since finding nothing also returns an error 
            // TODO: make better by making get_account option for that error specifically
        }
    }

    if account_infos.is_empty() {
        Err(ServerError::DBError(sqlx::Error::RowNotFound))
    } else {
        Ok(HttpResponse::Ok().json(RespPackage {
            accounts: account_infos,
        }))
    }
}