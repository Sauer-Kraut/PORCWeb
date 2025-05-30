use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::liberary::account_lib::account::pub_account_info::PubAccountInfo;
use crate::liberary::account_lib::account::storage::get_account::get_account;
use crate::liberary::account_lib::login::storage::get_login::get_login;
use crate::AppState;




#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    pub auth_key: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespPackage {
    pub account: PubAccountInfo,
}



// GET Request to receive the account belonging to the provided auth key
pub async fn get_login_request(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let query_auth_key = query.auth_key.clone();
    println!("auth key: {query_auth_key}");

    let login = get_login(query_auth_key, appstate.pool.clone()).await?;

    let account = get_account(login.account_id.clone(), appstate.pool.clone()).await?.get_pub_info();

    Ok(HttpResponse::Ok().json(RespPackage {
        account,
    }))
}