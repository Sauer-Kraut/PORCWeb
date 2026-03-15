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

    println!("Received request to get player ranking for season: {:?}", query.season);

    let matchplan = match query.season.clone() {
        None => {
            appstate.get_matchplan().await?
        },
        Some(s) =>  {
            get_matchplan(s, appstate.pool.clone()).await?
        }
    };

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
