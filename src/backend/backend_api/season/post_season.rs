use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::backend::backend_api::server_error::ServerError;
use crate::liberary::matchplan_lib::matchplan::storage::start_season::start_season;
use crate::liberary::matchplan_lib::matchplan_blueprint::matchplan_blueprint::PlanBlueprint;
use crate::AppState;



#[derive(Debug, Deserialize, Serialize)]
pub struct RecvPackage {
    pub plan: PlanBlueprint,
}

// POST Request to start a bew season
pub async fn post_new_season_request(info: web::Json<RecvPackage>, appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {

    let blueprint = info.plan.clone();
    check_blueprint(blueprint.clone())?;

    start_season(blueprint, appstate.pool.clone()).await?;

    Ok(HttpResponse::Ok())
}




pub fn check_blueprint(plan: PlanBlueprint) -> Result<(), Box<dyn std::error::Error>> {
    let mut players = Vec::new();
    let mut division_orders = Vec::new();
    
    let mut used_division_names = Vec::new();
    let acceptable_division_names = vec![
        "Meteorite", "Malachite", "Adamantium", "Mithril", "Platinum", 
        "Diamond", "Gold", "Silver", "Bronze", "Steel", "Copper", 
        "Iron", "Stone"
    ].iter().map(|f| f.to_string()).collect::<Vec<String>>();

    for division in plan.divisions.iter() {
        if !acceptable_division_names.contains(&division.name) {
            return Err(format!("Division name {} is not acceptable", division.name).into());
        }

        if used_division_names.contains(&division.name) {
            return Err(format!("Division name {} occurs multiple times", division.name).into());
        }

        used_division_names.push(division.name.clone());
    }

    if plan.divisions.len() < 2 {
        return Err("There are less than 2 divisions".to_string().into());
    }

    for division in plan.divisions.iter() {

        if division_orders.contains(&division.order) {
            return Err(format!("Division order {} occurs multiple times", division.order).into());
        }

        division_orders.push(division.order);

        if division.players.len() < 2 {
            return Err(format!("Division {} has less than 2 players", division.name).into());
        }

        for player in division.players.iter() {

            if players.contains(player) {
                return Err(format!("Player {:?} occurs multiple times", player).into());
            }

            players.push(player.clone());
        }
    }

    if plan.players_to_sort.len() != 0 {
        return Err("There are still players to sort".to_string().into());
    }

    if plan.pause_end_timestamp == None || plan.end_timestamp == None {
        return Err("Time stamps not correctly configured".to_string().into());
    }

    return Ok(());
}
