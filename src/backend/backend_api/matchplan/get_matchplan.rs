use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{backend::backend_api::server_error::ServerError, liberary::matchplan_lib::matchplan::{matchplan::MatchPlan, storage::matchplan_get::get_matchplan}, AppState};




#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    pub season: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RespPackage {
    pub plan: MatchPlan,
}


// GET Request to retrieve match plan for currrent season
pub async fn get_matchplan_request(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let season = match &query.season {
        Some(v) => v.clone(),
        None => {
            let season_opt = appstate.season.read().await.clone();
            match season_opt {
                Some(season) => season.name.clone(),
                None => {
                    return Err(ServerError::Other("No current season found".to_owned().into()));
                }
            }
        }
    };

    let matchplan = get_matchplan(season.clone(), appstate.pool.clone()).await?;

    Ok(HttpResponse::Ok().json(RespPackage {
        plan: matchplan
    }))
}
