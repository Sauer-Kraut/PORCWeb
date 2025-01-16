use serde::{Deserialize, Serialize};
use std::sync::mpsc;
use colored::Colorize;
use tokio;

use crate::{AppState, Match, MatchPlan, PlayerPerformance, StorageMod};
use actix_web::{web, Responder, HttpResponse};




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignUpInfo {
    pub username: String,
    pub bp: u32,
    pub region: String,
    pub discord_id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRequestPlanPackage {
    pub title: String,
    pub data: MatchPlan,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRequestSignUpPackage {
    pub title: String,
    pub data: Vec<SignUpInfo>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPlayerPerformancePackage {
    pub title: String,
    pub data: Vec<(String, Vec<PlayerPerformance>)>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostRequestMatchPackage {
    pub title: String,
    pub match_info: Match
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostRequestSignUpPackage {
    pub title: String,
    pub sing_up_info: SignUpInfo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostRequestReturnPackage {
    pub title: String,
    pub error: Option<String>
}



pub async fn get_match_plan_request(appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received GET Request for match plan".bold().cyan());

    let (data_sender, data_receiver) = mpsc::channel();
    let (error_sender, error_receiver) = mpsc::channel();
    let matchplan = appstate.matchplan.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let locked_matchplan = matchplan.lock().await;
        let matchplan: Option<MatchPlan> = locked_matchplan.clone();

        match matchplan {
            Some(plan) => {
                data_sender.send(plan.clone()).unwrap();
            },
            None => {
                error_sender.send("No Match Plan found".to_owned()).unwrap();
            }
        }

    }).await.unwrap();

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    let data = data_receiver.recv().unwrap();

    HttpResponse::Ok().json(GetRequestPlanPackage {
        title: "Server Match Plan Respons".to_string(),
        data,
        error
    })
}



pub async fn get_player_ranking_request(appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received GET Request for match plan".bold().cyan());

    let (data_sender, data_receiver) = mpsc::channel();
    let (error_sender, error_receiver) = mpsc::channel();
    let matchplan = appstate.matchplan.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let mut output = Vec::new();

        let locked_plan = matchplan.lock().await;
        let plan = match locked_plan.as_ref() {
            Some(plan) => plan,
            None => {
                error_sender.send("No Match Plan found".to_owned()).unwrap();
                return;
            }
        };

        for division in plan.divisions.iter() {
            
            let ranking = division.generate_perfomance().await;
            output.push((division.name.clone(), ranking));
        }

        data_sender.send(output).unwrap();

    }).await.unwrap();

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    let data = data_receiver.recv().unwrap();

    HttpResponse::Ok().json(GetPlayerPerformancePackage {
        title: "Server Player Performance Respons".to_string(),
        data,
        error
    })
}



pub async fn get_sign_up_request(appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received GET Request for sign ups".bold().cyan());

    let (data_sender, data_receiver) = mpsc::channel();
    let (error_sender, error_receiver) = mpsc::channel();
    let signups = appstate.signups.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let locked_signups = signups.lock().await;
        let signups: Vec<SignUpInfo> = locked_signups.clone();

        data_sender.send(signups).unwrap_or_else( |err| {
            error_sender.send(format!("Internal Server Error : {:?}", err)).unwrap();
        });

    }).await.unwrap();

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    let data = data_receiver.recv().unwrap();

    HttpResponse::Ok().json(GetRequestSignUpPackage {
        title: "Server Sign Ups Respons".to_string(),
        data,
        error
    })
}


pub async fn update_match_plan_request(info: web::Json<PostRequestMatchPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received POST Request for match plan".bold().cyan());

    let (error_sender, error_receiver) = mpsc::channel();
    let matchplan = appstate.matchplan.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let mut locked_matchplan = matchplan.lock().await;

        
        if locked_matchplan.is_some() {
            let mut plan = locked_matchplan.clone().unwrap();

            match plan.update_match(info.match_info.clone()) {
                Ok(_) => {
                    *locked_matchplan = Some(plan.clone());
                    match StorageMod::save_matchplan(plan.clone(), appstate.matchplan_path.as_str()) {
                        Ok(_) => {},
                        Err(err) => {
                            error_sender.send(err.to_string()).unwrap();
                        }
                    }
                },
                Err(err) => {
                    error_sender.send(err.to_string()).unwrap();
                }
            }
        } 
        
        else {
            error_sender.send("No match plan found".to_owned()).unwrap();
        }

    }).await.unwrap();

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    HttpResponse::Ok().json(PostRequestReturnPackage {
        title: "Server Match Plan Update Respons".to_string(),
        error
    })
}


pub async fn add_sign_up_request(info: web::Json<PostRequestSignUpPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received POST Request for sign up".bold().cyan());

    let (error_sender, error_receiver) = mpsc::channel();
    let signups = appstate.signups.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let mut locked_signups = signups.lock().await;
        let mut signups: Vec<SignUpInfo> = locked_signups.clone();

        for signup in signups.iter() {
            if sanetize_username(&signup.username) == sanetize_username(&info.sing_up_info.username) {
                error_sender.send(format!("Simular Sign Up already exists: {}", signup.username)).unwrap();
                return;
            }
        }

        signups.push(info.sing_up_info.clone());
        *locked_signups = signups.clone();
        match StorageMod::save_signups(signups.clone(), appstate.signups_path.as_str()) {
            Ok(_) => {},
            Err(err) => {
                error_sender.send(err.to_string()).unwrap();
            }
        }

    }).await.unwrap();

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    HttpResponse::Ok().json(PostRequestReturnPackage {
        title: "Server Sign Up Add Respons".to_string(),
        error
    })
}


pub async fn remove_sign_up_request(info: web::Json<PostRequestSignUpPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received POST Request for sign up".bold().cyan());

    let (error_sender, error_receiver) = mpsc::channel();
    let signups = appstate.signups.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {

        let mut locked_signups = signups.lock().await;
        let mut signups: Vec<SignUpInfo> = locked_signups.clone();

        let mut index_to_remove = None;

        for (index, signup) in signups.iter().enumerate() {
            if &signup.discord_id == &info.sing_up_info.discord_id {
                index_to_remove = Some(index);
            }
        }

        match index_to_remove {
            Some(index) => {

                signups.remove(index);
                *locked_signups = signups.clone();
                match StorageMod::save_signups(signups.clone(), appstate.signups_path.as_str()) {
                    Ok(_) => {},
                    Err(err) => {
                        error_sender.send(err.to_string()).unwrap();
                    }
                }
            },
            None => {
                error_sender.send("Sign up not found".to_string()).unwrap();
            }
        }

    }).await.unwrap();

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    HttpResponse::Ok().json(PostRequestReturnPackage {
        title: "Server Sign Up Remove Respons".to_string(),
        error
    })
}

pub fn sanetize_username(username: &str) -> String {
    username.to_lowercase().replace(" ", "")
    .chars().filter(|c| c.is_alphabetic()).collect()
}
