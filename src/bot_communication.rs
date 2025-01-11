use async_std::sync::Mutex;
use actix_web::FromRequest;
use actix_web::{web, HttpResponse, Responder};
use colored::Colorize;
use tokio;
use std::sync::{mpsc, Arc};
use serde::{Deserialize, Serialize};
use crate::{AppState, Division, GetRequestPlanPackage, GetRequestSignUpPackage, Player, SignUpInfo, sanetize_username};




#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanBlueprint {
    pub divisions: Vec<DivisionBlueprint>,
    pub players_to_sort: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DivisionBlueprint {
    pub name: String,
    pub order: usize,
    pub players: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRequestPlanBlueprintPackage {
    pub title: String,
    pub data: PlanBlueprint,
    pub error: Option<String>
}


pub async fn generate_plan_blueprint_request(appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received GET Request for plan blueprint".bold().cyan());

    let (data_sender, data_receiver) = mpsc::channel();
    let match_plan = appstate.matchplan.clone();
    let sign_ups = appstate.signups.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let plan_binding = match_plan.lock().await;
        let locked_plan = plan_binding.as_ref();
        let mut signup_binding = sign_ups.lock().await;
        let signups_locked: &mut Vec<SignUpInfo> = signup_binding.as_mut();

        match locked_plan {
            Some(plan) => {

                let mut division_blueprints: Vec<DivisionBlueprint> = Vec::new();
                let mut players_to_demote = Vec::new();

                for (index, division) in plan.divisions.iter().enumerate(){

                    let players = division.generate_perfomance().await;

                    let mut players_to_promote = Vec::new();
                    let mut players_to_keep = players_to_demote.clone();
                    players_to_demote.clear();

                    for (index, player) in players.iter().enumerate() {
                        if index as f32 <= division.players.len() as f32 / 3.0 {
                            players_to_promote.push(player.player.clone());
                        } 
                        else if index as f32 >= division.players.len() as f32 / 3.0 * 2.0 {
                            players_to_demote.push(player.player.clone());
                        } 
                        else {
                            players_to_keep.push(player.player.clone());
                        }
                    }

                    for player in players_to_promote.iter().filter(|player| {
                        for signup in signups_locked.iter(){
                            if sanetize_username(&signup.username) == sanetize_username(&player.tag) {
                                signups_locked.remove(signups_locked.iter().position(|x| x.username == signup.username).unwrap());
                                return true;
                            }
                        } 
                        return false;
                    }) {
                        match division_blueprints.get_mut(((index as i32 - 1 as i32).max(0 as i32)) as usize) {
                            Some(division_blueprint) => {
                                division_blueprint.players.push(player.tag.clone());
                            },
                            None => {
                                players_to_keep.push(player.clone());
                            }
                        }
                    }

                    let players = players_to_keep.iter().filter(|player| {
                        for signup in signups_locked.iter(){
                            if sanetize_username(&signup.username) == sanetize_username(&player.tag) {
                                signups_locked.remove(signups_locked.iter().position(|x| x.username == signup.username).unwrap());
                                return true;
                            }
                        } 
                        return false;
                    }).map(|player| player.tag.clone()).collect();

                    let division_blueprint = DivisionBlueprint {
                        name: division.name.clone(),
                        order: division.order,
                        players
                    };
                    
                    division_blueprints.push(division_blueprint);
                }

            data_sender.send(PlanBlueprint {
                divisions: division_blueprints,
                players_to_sort: signups_locked.iter().map(|signup| signup.username.clone()).collect()
            }).unwrap();

            },
            None => {
                data_sender.send(PlanBlueprint {
                    divisions: Vec::new(),
                    players_to_sort: signups_locked.iter().map(|signup| signup.username.clone()).collect()
                }).unwrap();
            }
        }

        

    }).await.unwrap();

    let data = data_receiver.recv().unwrap();

    let error = None;

    HttpResponse::Ok().json(GetRequestPlanBlueprintPackage {
        title: "Server match plan blueprint Respons".to_string(),
        data,
        error
    })
}



pub fn check_blueprint(plan: PlanBlueprint) -> Option<String> {
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
            return Some(format!("Division name {} is not acceptable", division.name));
        }

        if used_division_names.contains(&division.name) {
            return Some(format!("Division name {} occurs multiple times", division.name));
        }

        used_division_names.push(division.name.clone());
    }

    if plan.divisions.len() < 2 {
        return Some("There are less than 2 divisions".to_string());
    }

    for division in plan.divisions.iter() {

        if division_orders.contains(&division.order) {
            return Some(format!("Division order {} occurs multiple times", division.order));
        }

        division_orders.push(division.order);

        if division.players.len() < 2 {
            return Some(format!("Division {} has less than 2 players", division.name));
        }

        for player in division.players.iter() {

            if players.contains(player) {
                return Some(format!("Player {} occurs multiple times", player));
            }

            players.push(player.to_string());
        }
    }

    if plan.players_to_sort.len() != 0 {
        return Some("There are still players to sort".to_string());
    }

    return None;
}