use serde::{Deserialize, Serialize};
use colored::Colorize;

use crate::{liberary::{account_lib::signup::{signup::SignUpInfo, storage::{get_signups::get_signups, store_signup::store_signup}}, matchplan_lib::{division::player_performance::PlayerPerformance, matchplan::{matchplan::MatchPlan, storage::matchplan_get::get_matchplan}, matchplan_match::{matchplan_match::Match, storage::match_store::update_match}, season::storage::get_seasons}}, AppState};
use actix_web::{web, Responder, HttpResponse};



#[derive(Debug, Deserialize, Serialize)]
pub struct GetRequestPlanPackage {
    pub title: String,
    pub data: Option<MatchPlan>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetRequestSignUpPackage {
    pub title: String,
    pub data: Option<Vec<SignUpInfo>>,
    pub error: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPlayerPerformancePackage {
    pub title: String,
    pub data: Option<Vec<(String, Vec<PlayerPerformance>)>>,
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


// Request to retrieve match plan for currrent season
// TODO: will need to add fetch for other seasons later
pub async fn get_match_plan_request(appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received GET Request for match plan".bold().cyan());

    let result: Result<MatchPlan, String> = 'scope: {
        let seasons = get_seasons::get_seasons(appstate.pool.clone()).await.unwrap_or(vec!());

        if seasons.len() == 0 {
            break 'scope Err(format!("No Match Plan found"));
        }
        else {

            let current_season = seasons[0].clone();
            let matchplan = match get_matchplan(current_season.name.clone(), appstate.pool.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    break 'scope Err(format!("Couldnt get matchplan: {}", err));
                }
            };

            break 'scope Ok(matchplan);
        }
    };

    match result {
        Ok(data) => {
            return HttpResponse::Ok().json(GetRequestPlanPackage {
                title: "Server Match Plan Respons".to_string(),
                data: Some(data),
                error: None
            });
        },
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.red().bold());
            return HttpResponse::Ok().json(GetRequestPlanPackage {
                title: "Server Match Plan Respons".to_string(),
                data: None,
                error: Some(err)
            });
        }
    }
}



// Request to retrieve all player performance data for currrent season
// TODO: will need to add fetch for other seasons later
pub async fn get_player_ranking_request(appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received GET Request for match plan".bold().cyan());

    let result: Result<Vec<(String, Vec<PlayerPerformance>)>, String> = 'scope: {

        let seasons = get_seasons::get_seasons(appstate.pool.clone()).await.unwrap_or(vec!());

        if seasons.len() == 0 {
            break 'scope Err(format!("No Match Plan found"));
        }
        else {

            let current_season = seasons[0].clone();
            let matchplan = match get_matchplan(current_season.name.clone(), appstate.pool.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    break 'scope Err(format!("There was an error while getting the matchplan: {}", err));
                }
            };
            
            let divisions = matchplan.divisions;
            let mut data = vec!();

            for division in divisions {
                let performance = division.generate_perfomance().await;
                let res = (division.name, performance);
                data.push(res);
            }

            break 'scope Ok(data);
        }
    };
    
    match result {
        Ok(data) => {
            return HttpResponse::Ok().json(GetPlayerPerformancePackage {
                title: "Server Player Performance Respons".to_string(),
                data: Some(data),
                error: None
            });
        },
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.red().bold());
            return HttpResponse::Ok().json(GetPlayerPerformancePackage {
                title: "Server Player Performance Respons".to_string(),
                data: None,
                error: Some(err)
            });
        }
    }
}


// Request to retrieve all recent sign ups
pub async fn get_sign_up_request(appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received GET Request for sign ups".bold().cyan());

    let result: Result<Vec<SignUpInfo>, String> = 'scope: {
        let seasons = get_seasons::get_seasons(appstate.pool.clone()).await.unwrap_or(vec!());

        if seasons.len() < 1 {
            break 'scope Err(format!("No Match Plan found"));
        }
        else {

            let mut last_season_end_timestamp = 0;

            if seasons.len() > 1 {
                let last_season = seasons[1].clone();
                let matchplan = match get_matchplan(last_season.name.clone(), appstate.pool.clone()).await {
                    Ok(v) => v,
                    Err(err) => {
                        break 'scope Err(format!("There was an error while getting the matchplan: {}", err));
                    }
                };
                last_season_end_timestamp = matchplan.pause_end_timestamp;
            }
        
            let signups = match get_signups(last_season_end_timestamp, None, appstate.pool.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    break 'scope Err(format!("There was an error while getting the signups: {}", err));
                }
            };

            break 'scope Ok(signups);
        }
    };

    match result {
        Ok(data) => {
            return HttpResponse::Ok().json(GetRequestSignUpPackage {
                title: "Server Sign Up Respons".to_string(),
                data: Some(data),
                error: None
            });
        },
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.red().bold());
            return HttpResponse::Ok().json(GetRequestSignUpPackage {
                title: "Server Sign Up Respons".to_string(),
                data: None,
                error: Some(err)
            });
        }
    }
}


// Request to update a provided match of the current season
pub async fn update_match_plan_request(info: web::Json<PostRequestMatchPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received POST Request for match plan".bold().cyan());

    let result: Result<(), String> = 'scope: {

        let seasons = get_seasons::get_seasons(appstate.pool.clone()).await.unwrap_or(vec!());

        if seasons.len() < 1 {
            break 'scope Err(format!("No Match Plan found"));
        }
        else {
            let current_season = seasons[0].clone();
            
            match update_match(info.match_info.clone(), current_season.name, appstate.pool.clone()).await {
                Ok(_) => {
                    break 'scope Ok(());
                },
                Err(err) => {
                    break 'scope Err(format!("There was an error while updating the matchplan: {}", err));
                }
            };
        }
    };

    match result {
        Ok(_) => {
            return HttpResponse::Ok().json(PostRequestReturnPackage {
                title: "Server Match Plan Update Respons".to_string(),
                error: None
            });
        },
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.red().bold());
            return HttpResponse::Ok().json(PostRequestReturnPackage {
                title: "Server Match Plan Update Respons".to_string(),
                error: Some(err)
            });
        }
    }
}


// Request do add a sign up
pub async fn add_sign_up_request(info: web::Json<PostRequestSignUpPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received POST Request for sign up".bold().cyan());

    let result: Result<(), String> = 'scope: {
        match store_signup(info.sing_up_info.clone(), appstate.pool.clone()).await {
            Ok(_) => {
                break 'scope Ok(());
            },
            Err(err) => {
                break 'scope Err(format!("There was an error while adding the sign up: {}", err));
            }
        };
    };

    match result {
        Ok(_) => {
            return HttpResponse::Ok().json(PostRequestReturnPackage {
                title: "Server Sign Up Add Respons".to_string(),
                error: None
            });
        },
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.red().bold());
            return HttpResponse::Ok().json(PostRequestReturnPackage {
                title: "Server Sign Up Add Respons".to_string(),
                error: Some(err)
            });
        }
    }
}


// pub async fn remove_sign_up_request(info: web::Json<PostRequestSignUpPackage>, appstate: web::Data<AppState>) -> impl Responder {
//     println!("\n{}", "Received POST Request for sign up".bold().cyan());

//     let (error_sender, error_receiver) = mpsc::channel();
//     let signups = appstate.signups.clone();

//     // We spawn an asyncronus thread in order to be able to handle many requests at once
//     println!("startig async thread");
//     tokio::task::spawn(async move {

//         let mut locked_signups = signups.lock().await;
//         let mut signups: Vec<SignUpInfo> = locked_signups.clone();

//         let mut index_to_remove = None;

//         for (index, signup) in signups.iter().enumerate() {
//             if &signup.discord_id == &info.sing_up_info.discord_id {
//                 index_to_remove = Some(index);
//             }
//         }

//         match index_to_remove {
//             Some(index) => {

//                 signups.remove(index);
//                 *locked_signups = signups.clone();
//                 match StorageMod::save_signups(signups.clone()) {
//                     Ok(_) => {},
//                     Err(err) => {
//                         error_sender.send(err.to_string()).unwrap();
//                     }
//                 }
//             },
//             None => {
//                 error_sender.send("Sign up not found".to_string()).unwrap();
//             }
//         }

//     }).await.unwrap();

//     let error = match error_receiver.try_recv(){
//         Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
//         Err(_) => None,
//     };

//     HttpResponse::Ok().json(PostRequestReturnPackage {
//         title: "Server Sign Up Remove Respons".to_string(),
//         error
//     })
// }

pub fn sanetize_username(username: &str) -> String {
    username.to_lowercase().replace(" ", "")
    .chars().filter(|c| c.is_alphabetic()).collect()
}
