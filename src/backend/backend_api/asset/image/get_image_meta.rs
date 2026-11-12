use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::AppState;
use crate::liberary::account_lib::login::storage::get_login::get_login;
use crate::liberary::asset_lib::image::image_entry::ImageEntry;
use crate::liberary::asset_lib::image::storage::get_image_entries::get_image_entries;




#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    pub auth_key: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespPackage {
    pub image_entries: Vec<ImageEntry>,
}


// GET request to get image entries without schedule
pub async fn get_image_entries_request(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let login = get_login(query.auth_key.clone(), appstate.pool.clone()).await?;

    let image_entries = get_image_entries(login.account_id, None, appstate.pool.clone())
        .await?;

    if image_entries.is_empty() {
        Err(sqlx::Error::RowNotFound.into())
    } else {
        Ok(HttpResponse::Ok().json(RespPackage {
            image_entries: image_entries,
        }))
    }
}