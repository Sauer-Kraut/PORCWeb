mod backend;
mod porcbot;
pub mod liberary;

use actix_web::dev::Server;
use askama::filters::format;
use backend::backend_api::account::get_account_info::get_account_info_request;
use backend::backend_api::account::get_account_info_full::get_account_info_full_request;
use backend::backend_api::account::get_login::get_login_request;
use backend::backend_api::account::post_account_info::post_account_info_request;
use backend::backend_api::discord_communication::discord_callback;
use backend::backend_api::match_event::get_match_event::get_match_event_request;
use backend::backend_api::match_event::post_match_event::post_match_event_request;
use backend::backend_api::matchplan::get_matchplan::get_matchplan_request;
use backend::backend_api::matchplan::get_ranking::get_player_ranking_request;
use backend::backend_api::matchplan::post_match::post_match_request;
use backend::backend_api::season::get_blueprint::get_seasons_blueprint_request;
use backend::backend_api::season::get_seasons::get_seasons_request;
use backend::backend_api::season::post_season::post_new_season_request;
use backend::backend_api::server_error::ServerError;
use backend::backend_api::signup::get_signups::get_sign_ups_request;
use backend::backend_api::signup::post_signup::post_sign_up_request;

use async_std::fs;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::http;
use actix_files::Files;
use backend::storage_lib::{Config, StorageMod};
use backend::backend_api::middleware::ServerMiddleware;
use colored::Colorize;
use liberary::matchplan_lib::season::season::Season;
use liberary::matchplan_lib::season::storage::get_season::get_season;
use porcbot::config::{BOT_TOKEN, INTENTS};
use porcbot::tasks::events::bot_event_handler::BotEventHandler;
use porcbot::tasks::functions::check_dialogues::check_dialogues;
use serde::{Deserialize, Serialize};
use serenity::Client;
use sqlx::*;
use tokio;
use std::sync::{Arc};
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use dotenvy::dotenv;

use crate::backend::backend_api::discord::get_discord_events::get_discord_events_reqeust;
use crate::backend::backend_api::discord::get_discord_vods::get_discord_vods_reqeust;
use crate::liberary::discord_lib::discord_event::discord_event::DiscordEvent;
use crate::liberary::discord_lib::video_reference::video_reference::VideoReference;
use crate::liberary::matchplan_lib::division::player_performance::PlayerPerformance;
use crate::liberary::matchplan_lib::matchplan::matchplan::MatchPlan;
use crate::liberary::matchplan_lib::matchplan::storage::matchplan_get::get_matchplan;
use crate::liberary::util::functions::build_index::build_index;
use crate::porcbot::tasks::functions::collect_discord_vods::collect_discord_vods;
use crate::porcbot::tasks::functions::fetch_discord_events::fetch_discord_events;



#[derive(Serialize, Deserialize, Debug)]
struct InitialData {
    matchplan: MatchPlan,
    season: Season,
    ranking: Vec<(String, Vec<PlayerPerformance>)>,
    vods: Vec<VideoReference>,
    events: Vec<DiscordEvent>
}


// retruns the html site we want to serve
async fn index(appstate: web::Data<AppState>) -> Result<impl Responder, ServerError> {
    println!("\nYay, we got a request!");

    let data = InitialData {
        matchplan: appstate.get_matchplan().await?,
        season: appstate.get_season().await?,
        ranking: appstate.get_ranking().await?,
        vods: appstate.discord_vods.read().await.clone(),
        events: appstate.discord_events.read().await.clone(),
    };

    let argument_key = "{INITIALDATAPLACEHOLDER}";
    let json_contents = serde_json::to_string(&data).map_err(|e| e.to_string())?;

    let index = build_index(json_contents, fs::read_to_string("PORC-Front/dist/index.html").await.unwrap(), argument_key)?;
    println!("{}", index);
    Ok(HttpResponse::Ok().body(index))
}









#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // println!("{}", "This is a Prototype, please neither judge nor deploy. \nHi 2Guib by the way");
    println!("\n\n{}", "Starting server...");

    // let divisions: Vec<String> = vec!("Meteorite", "Malachite", "Adamantium", "Mithril", "Platinum", "Diamond", "Gold", "Silver", "Bronze", "Steel", "Copper", "Iron", "Stone").iter().map(|f| f.to_string()).collect();
    // let test_players: Vec<(String, String)> = vec!(("Tamrell", "Adamantium"), ("Sauerkraut", "Meteorite"), ("2Guib", "Meteorite"), ("Tomas", "Adamantium"), ("James", "Stone"), ("Pirate", "Stone"), ("Monkey", "Meteorite")).iter().map(|f| (f.0.to_string(), f.1.to_string())).collect();

    // let matchplan = MatchPlan::generate(test_players, divisions, false).unwrap();

    // println!("Test Matchplan: {}", matchplan);

    // let _ = StorageMod::save_matchplan(matchplan, "src/Season3MatchPlan.json")?;

    println!("reading config");
    let config = Arc::new(RwLock::new(StorageMod::read_config().unwrap()));
    let config_read = config.read().await.clone();

    println!("\n{}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}", "config was loaded:".to_string(), 
        "url:".cyan().bold(), config_read.url.cyan(),
        "domain:".cyan().bold(), config_read.domain.cyan(),
        "port:".cyan().bold(), config_read.port.cyan(),
        "season:".cyan().bold(), config_read.season.as_deref().unwrap_or("None").cyan(),
        "dev:".cyan().bold(), config_read.dev.to_string().cyan(),
    );
    let port = config.read().await.port.clone();

    // println!("read 1");
    // let read_plan = StorageMod::read_matchplan()?;

    // println!("read 2");
    // let logins = Arc::new(Mutex::new(StorageMod::read_logins()?));

    // // println!("Read Matchplan: {}", read_plan);

    // let matchplan = Arc::new(Mutex::new(Some(read_plan)));
    // // StorageMod::save_signups(vec!(), "src/Season4SignUps.json")?;
    
    // println!("read 3");
    // let signups = Arc::new(Mutex::new(StorageMod::read_signups()?));
    // // println!("secrets: {:?}", StorageMod::read_secrets().unwrap());

    // println!("read 4");
    // let accounts = Arc::new(Mutex::new(StorageMod::read_accounts()?));

    // println!("read 5");
    // let matchevents = Arc::new(Mutex::new(StorageMod::read_matchevents()?));

    // println!("read 7");
    // let dialogues = Arc::new(Mutex::new(StorageMod::read_dialogues()?));

    dotenv().ok();

    let pool = match config.read().await.dev {
        true => PgPool::connect(&std::env::var("DEV_DATABASE_URL").unwrap()).await.unwrap(), // dev database
        false => PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap(), // production database
    };

    let current_season = if let None = config_read.season {
        None
    } 
    else {
        match get_season(config_read.season.unwrap(), pool.clone()).await {
            Ok(v) => v,
            Err(_) => None,
        }
    };

    let season = Arc::new(RwLock::new(current_season));

    // spawns a loop to check config and current season
    tokio::spawn({
        let config = config.clone();
        let season = season.clone();
        let pool = pool.clone();
        async move {
            loop {
                // println!("updating config");
                let current_config = StorageMod::read_config().unwrap();

                // Compare the values inside the lock, not the guards/arcs
                let config_read = config.read().await;
                if *config_read != current_config {
                    drop(config_read); // Release read lock before acquiring write lock
                    let mut config_lock = config.write().await;
                    *config_lock = current_config.clone();

                    // Ensure both branches return Result<Option<Season>, ...>
                    let current_season_result = if let Some(season_name) = current_config.season.clone() {
                        get_season(season_name, pool.clone()).await
                    } else {
                        Ok(None)
                    };

                    match current_season_result {
                        Ok(season_opt) => {
                            let mut season_lock = season.write().await;
                            *season_lock = season_opt;
                        },
                        Err(e) => {
                            println!("Error getting season: {:?}", e);
                        }
                    }

                    println!("config was updated, new config: {current_config:?}");
                }
                sleep(Duration::from_secs(120)).await;
            }
        }
    });

    let appstate = AppState {
        // matchplan,
        // signups,
        // logins,
        // accounts,
        // matchevents,
        // dialogues,
        discord_events: Arc::new(RwLock::new(vec!())),
        discord_vods: Arc::new(RwLock::new(vec!())),
        matchplan: Arc::new(RwLock::new(None)),
        ranking: Arc::new(RwLock::new(None)),
        season: season.clone(),
        config: config.clone(),
        pool
    };

    let appstate_clone = appstate.clone();

    // spawns bot dialogue checker loop
    tokio::spawn(async move {

        let appstate_clone_2 = appstate_clone.clone();

        let _dialogue_task = tokio::task::spawn(async move {
            println!("\n{}", "Bot dialogue check loop has has launched".bright_green());
            loop {
                // println!("checking active dialogues");
                match check_dialogues(&appstate_clone).await {
                    Ok(_) => (),
                    Err(err) => println!("{}", format!("An error has occured while checking active dialogues: {err}").red()),
                }
                // println!("finished checking active dialogues");
                sleep(Duration::from_secs(30)).await; // waits 30 seconds between each loop
            }
        });

        let _bot_task = tokio::task::spawn(async move {
            let token = BOT_TOKEN.as_ref().clone();
            let intents = INTENTS.as_ref().clone();

            let mut client = Client::builder(token, intents)
                .event_handler(BotEventHandler{
                    appstate: appstate_clone_2
                })
                .await
                .expect("Error creating client");

            if let Err(why) = client.start().await {
                println!("Client error: {:?}", why);
            }
        });
    });

    let appstate_clone_2 = appstate.clone();

    // spawns a loop to fetch discord events and vods
    tokio::spawn(async move {
        let appstate_clone = appstate_clone_2.clone();
        let _event_task = tokio::task::spawn(async move {
            println!("\n{}", "Discord event fetch loop has has launched".bright_green());
            loop {
                match fetch_discord_events(&appstate_clone).await {
                    Ok(events) => {
                        let mut events_lock = appstate_clone.discord_events.write().await;
                        *events_lock = events;
                    },
                    Err(err) => println!("{}", format!("An error has occured while fetching discord events: {err}").red()),
                }
                sleep(Duration::from_secs(300)).await; // waits 5 minutes between each loop
            }
        });

        let appstate_clone = appstate_clone_2;
        let _vod_task = tokio::task::spawn(async move {
            println!("\n{}", "Discord vod fetch loop has has launched".bright_green());
            loop {
                match collect_discord_vods(&appstate_clone, 50).await {
                    Ok(vods) => {
                        let mut vods_lock = appstate_clone.discord_vods.write().await;
                        *vods_lock = vods;
                    },
                    Err(err) => println!("{}", format!("An error has occured while fetching discord vods: {err}").red()),
                }
                sleep(Duration::from_secs(300)).await; // waits 5 minutes between each loop
            }
        });
    });

    let appstate_clone_3 = appstate.clone();

    // loop to regularly refresh matchplan and rankings
    tokio::spawn(async move {
        let _event_task = tokio::task::spawn(async move {
            println!("\n{}", "matchplan refresh loop has has launched".bright_green());
            loop {
                match appstate_clone_3.refresh_season_info().await {
                    Ok(_) => {},
                    Err(err) => println!("{}", format!("An error has occured while refreshing matchplan: {err}").red()),
                }
                sleep(Duration::from_secs(120)).await; // waits 5 minutes between each loop
            }
        });
    });

    println!("\n{}", "Server has launched".bright_white());

    let port_clone = port.clone();
    let url = config.read().await.url.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                .allowed_origin(&format!("{}{}", "http://localhost:", port_clone)) 
                .allowed_origin("http://localhost:5173") 
                .allowed_origin(&url) // Update with your frontend's origin
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT,
                    http::header::ORIGIN,
                    http::header::CONTENT_TYPE
                ])
                .allow_any_header()
                .supports_credentials()
                .max_age(3600))
            .wrap(ServerMiddleware)
            .app_data(web::Data::new(appstate.clone()))
            .service(web::resource("/").to(index))
            .service(web::resource("/signup").to(index))
            .service(web::resource("/rules").to(index))
            .service(web::resource("/faq").to(index))
            .service(web::resource("/match-planner").to(index))

            // /api/account

            .service(web::resource("/api/account/simple")
            .route(web::get().to(get_account_info_request)))

            .service(web::resource("/api/account/full")
            .route(web::get().to(get_account_info_full_request)))

            .service(web::resource("/api/account/login")
            .route(web::get().to(get_login_request)))

            .service(web::resource("/api/account/update")
            .route(web::post().to(post_account_info_request)))

            // /api/matchplan

            .service(web::resource("/api/matchplan")
            .route(web::get().to(get_matchplan_request)))
            
            .service(web::resource("/api/matchplan/match")
            .route(web::post().to(post_match_request)))

            .service(web::resource("/api/matchplan/ranking")
            .route(web::get().to(get_player_ranking_request)))

            // /api/signup

            .service(web::resource("/api/sign-up")
            .route(web::get().to(get_sign_ups_request))
            .route(web::post().to(post_sign_up_request)))

            // /api/season

            .service(web::resource("/api/season")
            .route(web::get().to(get_seasons_request)))

            .service(web::resource("/api/season/controll")
            .route(web::get().to(get_seasons_blueprint_request))
            .route(web::post().to(post_new_season_request)))

            // /api/match-event

            .service(web::resource("/api/match-event")
            .route(web::get().to(get_match_event_request))
            .route(web::post().to(post_match_event_request)))

            // /api/query-testing

            .service(web::resource("/api/query-testing")
            .route(web::get().to(get_account_info_request)))

            // /api/discord

            .service(web::resource("/api/discord/events")
            .route(web::get().to(get_discord_events_reqeust)))

            .service(web::resource("/api/discord/vods")
            .route(web::get().to(get_discord_vods_reqeust)))


            .service(web::resource("/discord/callback").to(discord_callback))
            .service(Files::new("/", "./PORC-Front/dist").index_file("index.html"))
    })
    .bind(&format!("{}{}", "[::]:", port))? // Production port: 8081, devolpment sever port: 8082, local port:8082
    // .bind(&format!("{}{}", "0.0.0.0:", port))?  // surely this wont fuck clients who want to bind with ipv6
    .run()
    .await
}


#[derive(Clone)]
pub struct AppState {
    // matchplan: Arc<Mutex<Option<MatchPlan>>>,
    // signups: Arc<Mutex<Vec<SignUpInfo>>>,
    // logins: Arc<Mutex<HashMap<String, String>>>,
    // accounts: Arc<Mutex<HashMap<String, Account>>>,
    // matchevents: Arc<Mutex<HashMap<String, MatchEvent>>>,
    // dialogues: Arc<Mutex<Vec<DialogueBuilder>>>,
    discord_events: Arc<RwLock<Vec<DiscordEvent>>>,
    discord_vods: Arc<RwLock<Vec<VideoReference>>>,
    matchplan: Arc<RwLock<Option<MatchPlan>>>,    // needs to be refreshed every time its updated
    season: Arc<RwLock<Option<Season>>>,
    ranking: Arc<RwLock<Option<Vec<(String, Vec<PlayerPerformance>)>>>>,
    config: Arc<RwLock<Config>>,
    pool: Pool<Postgres>
}

impl AppState {

    pub async fn get_season(&self) -> Result<Season, ServerError> {

        match self.season.read().await.as_ref() {
            Some(s) => return Ok(s.clone()),
            None => return Err(ServerError::Other("current season could not be found".into())),
        }
    }

    pub async fn get_matchplan(&self) -> Result<MatchPlan, ServerError> {

        match self.matchplan.read().await.as_ref() {
            Some(plan) => {
                Ok(plan.clone())
            },

            None => {
                
                match self.refresh_season_info().await? {
                    Some(res) => {
                        Ok(res.0)
                    },
                    None => {
                        Err("couldnt find current matchplan".into())
                    }
                }
            }
        }
    }

    pub async fn get_ranking(&self) -> Result<Vec<(String, Vec<PlayerPerformance>)>, ServerError> {

        match self.ranking.read().await.as_ref() {
            Some(ranking) => {
                Ok(ranking.clone())
            },

            None => {
                
                match self.refresh_season_info().await? {
                    Some(res) => {
                        Ok(res.1)
                    },
                    None => {
                        Err("couldnt find current ranking".into())
                    }
                }
            }
        }
    }

    // ranking and matchplan grouped together to avoid faulty partial refreshing
    // perfomance wise not optimal to always returned cloned values but it makes my life easier
    pub async fn refresh_season_info(&self) -> Result<Option<(MatchPlan, Vec<(String, Vec<PlayerPerformance>)>)>, ServerError> {
        let mut plan = self.matchplan.write().await;
        *plan = None;

        let mut ranking = self.ranking.write().await;
        *ranking = None;

        match self.get_season().await {
            Ok(s) => {
                match get_matchplan(s.name, self.pool.clone()).await {
                    Ok(m) => {

                        let divisions = m.divisions.clone();
                        let mut rankings = vec!();

                        for division in divisions {
                            let performance = division.generate_perfomance().await;
                            let res = (division.name, performance);
                            rankings.push(res);
                        }

                        (*plan, *ranking) = (Some(m.clone()), Some(rankings.clone()));
                        return Ok(Some((m, rankings)))
                    },
                    _ => ()
                }
            }
            _ => ()
        }
        Ok(None)   
    }
}

// impl AppState {

//     pub async fn refresh(&self) {
//         println!("\nRefreshing appstate... ");

//         let accounts_clone = self.accounts.clone();
//         let accounts = accounts_clone.lock().await;

//         let matchplan_clone = self.matchplan.clone();
//         let mut matchplan_lock = matchplan_clone.lock().await;

//         if let Some(matchplan) = matchplan_lock.as_mut() {
//             println!("Refreshing matchplan... ");
//             for division in matchplan.divisions.iter_mut() {
//                 println!("Refreshing division {}... ", division.name);

//                 for (_key, value) in division.matches.iter_mut() {

//                     let account_p1 = accounts.get(&value.p1.id);

//                     match account_p1 {
//                         Some(account) => {
//                             if account.user_info.username != value.p1.tag {
//                                 value.p1.tag = account.user_info.username.clone();
//                             }
//                         },
//                         None => {},
//                     }

//                     let account_p2 = accounts.get(&value.p2.id);

//                     match account_p2 {
//                         Some(account) => {
//                             if account.user_info.username != value.p2.tag {
//                                 value.p2.tag = account.user_info.username.clone();
//                             }
//                         },
//                         None => {},
//                     }
//                 }

//                 for player in division.players.iter_mut() {

//                     player.tag = accounts.get(&player.id).unwrap_or(
//                         &Account { user_info: DiscordUser { id: 0, username: "".to_string(), discriminator: 0, avatar: None, email: None}, schedule: None}
//                     ).user_info.username.clone()
//                 }
//             }

//             for player in matchplan.players.iter_mut() {
//                 player.tag = accounts.get(&player.id).unwrap_or(
//                     &Account { user_info: DiscordUser { id: 0, username: "".to_string(), discriminator: 0, avatar: None, email: None}, schedule: None}
//                 ).user_info.username.clone()
//             }

//             let res = StorageMod::save_matchplan(matchplan.clone());
//             let res = StorageMod::save_logins(self.logins.lock().await.clone());
//             let res_dis = match res {
//                 Ok(_) => "Ok".green(),
//                 Err(err) => format!("Error: {err:?}").red(),
//             };
//             println!("Saving matchplan data: {}", res_dis);
//         }

//         let res = StorageMod::save_accounts(accounts.clone());
//         let res_dis = match res {
//             Ok(_) => "Ok".green(),
//             Err(err) => format!("Error: {err:?}").red(),
//         };
//         println!("Saving account data: {}", res_dis);
//         let res = StorageMod::save_logins(self.logins.lock().await.clone());
//         let res_dis = match res {
//             Ok(_) => "Ok".green(),
//             Err(err) => format!("Error: {err:?}").red(),
//         };
//         println!("Saving login data: {}", res_dis);
//         let res = StorageMod::save_signups(self.signups.lock().await.clone());
//         let res_dis = match res {
//             Ok(_) => "Ok".green(),
//             Err(err) => format!("Error: {err:?}").red(),
//         };
//         println!("Saving signup data: {}", res_dis);
//         let res = StorageMod::save_matchevents(self.matchevents.lock().await.clone());
//         let res_dis = match res {
//             Ok(_) => "Ok".green(),
//             Err(err) => format!("Error: {err:?}").red(),
//         };
//         println!("Saving matchevent data: {}", res_dis);
//         let res = StorageMod::save_dialogues(self.dialogues.lock().await.clone());
//         let res_dis = match res {
//             Ok(_) => "Ok".green(),
//             Err(err) => format!("Error: {err:?}").red(),
//         };
//         println!("Saving dialogue data: {}\n", res_dis);
//     }

//     pub async fn refresh_dialogues(&self) {
//         println!("\nRefreshing dialogues... ");
//         let res = StorageMod::save_dialogues(self.dialogues.lock().await.clone());
//         let res_dis = match res {
//             Ok(_) => "Ok".green(),
//             Err(err) => format!("Error: {err:?}").red(),
//         };
//         println!("Saving dialogue data: {}\n", res_dis);
//     }
// }