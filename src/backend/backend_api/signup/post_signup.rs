use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::liberary::account_lib::signup::signup::SignUpInfo;
use crate::liberary::account_lib::signup::storage::store_signup::store_signup;
use crate::AppState;



#[derive(Debug, Deserialize, Serialize)]
pub struct RecvPackage {
    pub signup: SignUpInfo,
}

// POST Request do add a sign up
pub async fn post_sign_up_request(info: web::Json<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    store_signup(info.signup.clone(), appstate.pool.clone()).await?;

    Ok(HttpResponse::Ok())
}