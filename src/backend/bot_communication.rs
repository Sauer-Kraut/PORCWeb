use actix_web::{web, HttpResponse, Responder};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::liberary::account_lib::match_event::match_event::MatchEvent;
use crate::liberary::account_lib::signup::storage::get_signups::get_signups;
use crate::liberary::dialogue_lib::dialogue_builder::storage::store_dialogue::store_dialogue;
use crate::liberary::dialogue_lib::dialogue_initiator::dialogue_initiator::DialogueInitator;
use crate::liberary::matchplan_lib::matchplan::storage::matchplan_get::get_matchplan;
use crate::liberary::matchplan_lib::matchplan::storage::start_season::start_season;
use crate::liberary::matchplan_lib::matchplan_blueprint::matchplan_blueprint::PlanBlueprint;
use crate::liberary::matchplan_lib::season::storage::get_seasons::get_seasons;
use crate::AppState;





#[derive(Debug, Deserialize, Serialize)]
pub struct GetRequestPlanBlueprintPackage {
    pub title: String,
    pub data: Option<PlanBlueprint>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateNewSeasonRecvPackage {
    pub title: String,
    pub plan: PlanBlueprint,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateNewSeasonSendPackage {
    pub title: String,
    pub error: Option<String>
}


pub async fn generate_plan_blueprint_request(appstate: web::Data<AppState>) -> impl Responder {

    let mut error = None;

    let pool = appstate.pool.clone();

    let seasons = get_seasons(pool.clone()).await.unwrap_or(vec!());
    let mut data = None;


    if seasons.len() > 0 {
        
        let current_season = seasons[0].clone();
        let matchplan = match get_matchplan(current_season.name.clone(), pool.clone()).await {
            Ok(v) => v,
            Err(err) => {
                error = Some(format!("There was an error while getting the matchplan: {}", err));
                return HttpResponse::InternalServerError().json(GetRequestPlanBlueprintPackage {
                    title: "Server match plan blueprint Respons".to_string(),
                    data,
                    error
                });
            }
        };

        let mut last_season_end_timestamp = 0;
        if seasons.len() > 1 {

            let last_season = seasons[1].clone();
            let last_matchplan = match get_matchplan(last_season.name.clone(), pool.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    error = Some(format!("There was an error while getting the matchplan: {}", err));
                    return HttpResponse::InternalServerError().json(GetRequestPlanBlueprintPackage {
                        title: "Server match plan blueprint Respons".to_string(),
                        data,
                        error
                    });
                }
            };

            last_season_end_timestamp = last_matchplan.pause_end_timestamp;
        }

        let signups = match get_signups(last_season_end_timestamp, None, pool.clone()).await {
            Ok(v) => v,
            Err(err) => {
                error = Some(format!("There was an error while getting the signups: {}", err));
                return HttpResponse::InternalServerError().json(GetRequestPlanBlueprintPackage {
                    title: "Server match plan blueprint Respons".to_string(),
                    data,
                    error
                });
            }
        };

        let blueprint = matchplan.generate_blueprint(signups.clone()).await;

        return HttpResponse::Ok().json(GetRequestPlanBlueprintPackage {
            title: "Server match plan blueprint Respons".to_string(),
            data: Some(blueprint),
            error: None
        });
    }
    else {
        HttpResponse::Ok().json(GetRequestPlanBlueprintPackage {
            title: "Server match plan blueprint Respons".to_string(),
            data: None,
            error: Some("no seasons available".to_string())
        })
    }

    
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



pub async fn start_new_season(info: web::Json<GenerateNewSeasonRecvPackage>, appstate: web::Data<AppState>) -> impl Responder {

    let mut error = None;

    let blueprint = info.plan.clone();

    check_blueprint(blueprint.clone()).unwrap(); // Not meant for porduction

    match start_season(blueprint, appstate.pool.clone()).await {
        Ok(_) => {},
        Err(err) => {
            error = Some(format!("There was an error while starting the new season: {}", err));
            println!("{} {}", "An Error occured:".red().bold(), error.clone().unwrap_or("".to_string()).red().bold());
            return HttpResponse::Ok().json(GenerateNewSeasonSendPackage {
                title: "Server New Season start Respons".to_string(),
                error
            });
        }
    };

    HttpResponse::Ok().json(GenerateNewSeasonSendPackage {
        title: "Server New Season start Respons".to_string(),
        error
    })
}