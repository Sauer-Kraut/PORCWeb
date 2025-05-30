use actix_web::{web, HttpResponse, Responder};
use futures::join;
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError; 
use crate::liberary::account_lib::signup::storage::get_signups::get_signups;
use crate::liberary::matchplan_lib::matchplan::storage::matchplan_get::get_matchplan;
use crate::liberary::matchplan_lib::matchplan_blueprint::matchplan_blueprint::PlanBlueprint;
use crate::AppState;




#[derive(Serialize, Deserialize, Debug)]
pub struct RespPackage {
    pub blueprint: PlanBlueprint
}

// GER request to retrieve season blueprint based on the current season
pub async fn get_seasons_blueprint_request(appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let pool = appstate.pool.clone();
    let season = appstate.get_season().await?;
        
    let matchplan_fut = get_matchplan(season.name, pool.clone());
    let signups_fut = get_signups(season.start_timestamp, Some(season.pause_end_timestamp), pool.clone());

    let (matchplan_res, signup_res) = join!(matchplan_fut, signups_fut);
    let (matchplan, signups) = (matchplan_res?, signup_res?);

    let blueprint = matchplan.generate_blueprint(signups).await;

    Ok(HttpResponse::Ok().json(RespPackage {
        blueprint
    }))    
}
