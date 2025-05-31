use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError; 
use crate::liberary::matchplan_lib::division::player_performance::PlayerPerformance;
use crate::liberary::matchplan_lib::matchplan::storage::matchplan_get::get_matchplan;
use crate::AppState;






#[derive(Serialize, Deserialize, Debug)]
pub struct RecvPackage {
    pub season: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RespPackage {
    pub rankings: Vec<(String, Vec<PlayerPerformance>)>,
}


// GET Request to retrieve all player performance data for currrent season
pub async fn get_player_ranking_request(query: web::Query<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

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

    let divisions = matchplan.divisions;
    let mut rankings = vec!();

    for division in divisions {
        let performance = division.generate_perfomance().await;
        let res = (division.name, performance);
        rankings.push(res);
    }

    Ok(HttpResponse::Ok().json(RespPackage {
        rankings
    }))
}
