use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::backend::backend_api::util::parse_query_param::parse_query_param_to_vec;
use crate::AppState;
use crate::liberary::asset_lib::image::image_entry::ImageEntry;
use crate::liberary::asset_lib::image::storage::get_image_entry_by_id::get_image_entries_by_id;




#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    pub ids: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespPackage {
    pub image_entries: Vec<ImageEntry>,
}


// GET request to get image entries without schedule
pub async fn get_image_entries_by_id_request(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let query_ids = parse_query_param_to_vec::<String>(query.ids.clone())?;

    let image_entries = get_image_entries_by_id(query_ids, appstate.pool.clone())
        .await?;

    if image_entries.is_empty() {
        Err(sqlx::Error::RowNotFound.into())
    } else {
        Ok(HttpResponse::Ok().json(RespPackage {
            image_entries: image_entries,
        }))
    }
}