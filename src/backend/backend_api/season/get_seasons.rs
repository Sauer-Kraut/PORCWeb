use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError; 
use crate::liberary::matchplan_lib::season::storage::get_seasons::get_seasons;
use crate::liberary::matchplan_lib::season::season::*;
use crate::AppState;






#[derive(Debug, Deserialize, Serialize)]
pub struct RespPackage {
    pub seasons: Vec<Season>,
}


// GET Request to retrieve all seasons
// returns all seasons as fist -> last
pub async fn get_seasons_request(appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let mut seasons = get_seasons(appstate.pool.clone()).await?;
    seasons.sort_by(|a, b| a.start_timestamp.cmp(&b.start_timestamp));

    Ok(HttpResponse::Ok().json(RespPackage {
        seasons
    }))
}
