mod backend;
mod porcbot;

use backend::data_lib::*;
use backend::discord_communication::{discord_callback, put_logged_in};
use backend::account_lib::*;
use backend::storage_lib::*;
use backend::client_communication::*;
use backend::bot_communication::*;

use async_std::sync::Mutex;
use async_std::fs;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::http;
use actix_files::Files;
use colored::Colorize;
use porcbot::config::{BOT_TOKEN, INTENTS};
use porcbot::dialogue_module::dialogue_builder::DialogueBuilder;
use porcbot::tasks::events::bot_event_handler::BotEventHandler;
use porcbot::tasks::functions::check_dialogues::check_dialogues;
use serenity::Client;
use tokio;
use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};






// retruns the html site we want to serve
async fn index() -> impl Responder {
    println!("\nYay, we got a request!");
    HttpResponse::Ok().body(fs::read_to_string("PORC-Front/dist/index.html").await.unwrap())
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

    println!("read 1");
    let read_plan = StorageMod::read_matchplan()?;

    println!("read 2");
    let logins = Arc::new(Mutex::new(StorageMod::read_logins()?));

    // println!("Read Matchplan: {}", read_plan);

    let matchplan = Arc::new(Mutex::new(Some(read_plan)));
    // StorageMod::save_signups(vec!(), "src/Season4SignUps.json")?;
    println!("read 3");
    let signups = Arc::new(Mutex::new(StorageMod::read_signups()?));
    // println!("secrets: {:?}", StorageMod::read_secrets().unwrap());

    println!("read 4");
    let accounts = Arc::new(Mutex::new(StorageMod::read_accounts()?));

    println!("read 5");
    let matchevents = Arc::new(Mutex::new(StorageMod::read_matchevents()?));

    println!("read 6");
    let config = Arc::new(StorageMod::read_config()?);
    let port = config.port.clone();

    println!("read 7");
    let dialogues = Arc::new(Mutex::new(StorageMod::read_dialogues()?));

    let appstate = AppState {
        matchplan,
        signups,
        logins,
        accounts,
        matchevents,
        config: config.clone(),
        dialogues
    };

    let appstate_clone = appstate.clone();

    // spawns bot dialogue checker loop
    tokio::spawn(async move {

        let appstate_clone_2 = appstate_clone.clone();

        let _dialogue_task = tokio::task::spawn(async move {
            println!("\n{}", "Bot dialogue check loop has launched");
            loop {
                println!("checking active dialogues");
                match check_dialogues(&appstate_clone).await {
                    Ok(_) => (),
                    Err(err) => println!("{}", format!("An error has occured while checking active dialogues: {err}").red()),
                }
                println!("finished checking active dialogues");
                sleep(Duration::from_secs(360)).await; // waits 6 minutes between each loop
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

    println!("\n{}", "Server has launched".bright_white());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                .allowed_origin(&format!("{}{}", "http://localhost:", config.port.clone())) 
                .allowed_origin("http://localhost:5173") 
                .allowed_origin(&config.url.clone()) // Update with your frontend's origin
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
            .app_data(web::Data::new(appstate.clone()))
            .service(web::resource("/").to(index))
            .service(web::resource("/signup").to(index))
            .service(web::resource("/rules").to(index))
            .service(web::resource("/faq").to(index))
            .service(web::resource("/faq").to(index))
            .service(web::resource("/match-planner").to(index))
            .service(web::resource("/api/match-plan")
            .route(web::get().to(get_match_plan_request))
            .route(web::post().to(update_match_plan_request)))
            .service(web::resource("/api/sign-up")
            .route(web::get().to(get_sign_up_request))
            .route(web::post().to(add_sign_up_request)))
            .service(web::resource("/api/sign-up/remove")
            .route(web::post().to(remove_sign_up_request)))
            .service(web::resource("/api/ranking")
            .route(web::get().to(get_player_ranking_request)))
            .service(web::resource("/api/plan-blueprint")
            .route(web::get().to(generate_plan_blueprint_request)))
            .service(web::resource("/api/season-control")
            .route(web::post().to(start_new_season)))
            .service(web::resource("/discord/callback").to(discord_callback))
            .service(web::resource("/api/discord/logged-in")
            .route(web::put().to(put_logged_in)))
            .service(web::resource("/api/account/info")
            .route(web::put().to(put_account_info)))
            .service(web::resource("/api/account/setinfo")
            .route(web::post().to(post_account_info)))
            .service(web::resource("/api/matches/set")
            .route(web::post().to(post_match_event)))
            .service(web::resource("/api/matches/get")
            .route(web::put().to(put_match_event)))
            .service(Files::new("/", "./PORC-Front/dist").index_file("index.html"))

    })
    .bind(&format!("{}{}", "[::]:", port))? // Production port: 8081, devolpment sever port: 8082, local port:8082
    .run()
    .await
}


#[derive(Clone)]
pub struct AppState {
    matchplan: Arc<Mutex<Option<MatchPlan>>>,
    signups: Arc<Mutex<Vec<SignUpInfo>>>,
    logins: Arc<Mutex<HashMap<String, String>>>,
    accounts: Arc<Mutex<HashMap<String, Account>>>,
    matchevents: Arc<Mutex<HashMap<String, MatchEvent>>>,
    dialogues: Arc<Mutex<Vec<DialogueBuilder>>>,
    config: Arc<Config>
}

impl AppState {

    pub async fn refresh(&self) {
        println!("\nRefreshing appstate... ");

        let accounts_clone = self.accounts.clone();
        let accounts = accounts_clone.lock().await;

        let matchplan_clone = self.matchplan.clone();
        let mut matchplan_lock = matchplan_clone.lock().await;

        if let Some(matchplan) = matchplan_lock.as_mut() {
            println!("Refreshing matchplan... ");
            for division in matchplan.divisions.iter_mut() {
                println!("Refreshing division {}... ", division.name);

                for (_key, value) in division.matches.iter_mut() {

                    let account_p1 = accounts.get(&value.p1.id);

                    match account_p1 {
                        Some(account) => {
                            if account.user_info.username != value.p1.tag {
                                value.p1.tag = account.user_info.username.clone();
                            }
                        },
                        None => {},
                    }

                    let account_p2 = accounts.get(&value.p2.id);

                    match account_p2 {
                        Some(account) => {
                            if account.user_info.username != value.p2.tag {
                                value.p2.tag = account.user_info.username.clone();
                            }
                        },
                        None => {},
                    }
                }

                for player in division.players.iter_mut() {

                    player.tag = accounts.get(&player.id).unwrap_or(
                        &Account { user_info: DiscordUser { id: "".to_string(), username: "".to_string(), discriminator: "".to_string(), avatar: None, email: None}, schedule: None}
                    ).user_info.username.clone()
                }
            }

            for player in matchplan.players.iter_mut() {
                player.tag = accounts.get(&player.id).unwrap_or(
                    &Account { user_info: DiscordUser { id: "".to_string(), username: "".to_string(), discriminator: "".to_string(), avatar: None, email: None}, schedule: None}
                ).user_info.username.clone()
            }

            let res = StorageMod::save_matchplan(matchplan.clone());
            let res = StorageMod::save_logins(self.logins.lock().await.clone());
            let res_dis = match res {
                Ok(_) => "Ok".green(),
                Err(err) => format!("Error: {err:?}").red(),
            };
            println!("Saving matchplan data: {}", res_dis);
        }

        let res = StorageMod::save_accounts(accounts.clone());
        let res_dis = match res {
            Ok(_) => "Ok".green(),
            Err(err) => format!("Error: {err:?}").red(),
        };
        println!("Saving account data: {}", res_dis);
        let res = StorageMod::save_logins(self.logins.lock().await.clone());
        let res_dis = match res {
            Ok(_) => "Ok".green(),
            Err(err) => format!("Error: {err:?}").red(),
        };
        println!("Saving login data: {}", res_dis);
        let res = StorageMod::save_signups(self.signups.lock().await.clone());
        let res_dis = match res {
            Ok(_) => "Ok".green(),
            Err(err) => format!("Error: {err:?}").red(),
        };
        println!("Saving signup data: {}", res_dis);
        let res = StorageMod::save_matchevents(self.matchevents.lock().await.clone());
        let res_dis = match res {
            Ok(_) => "Ok".green(),
            Err(err) => format!("Error: {err:?}").red(),
        };
        println!("Saving matchevent data: {}", res_dis);
        let res = StorageMod::save_dialogues(self.dialogues.lock().await.clone());
        let res_dis = match res {
            Ok(_) => "Ok".green(),
            Err(err) => format!("Error: {err:?}").red(),
        };
        println!("Saving dialogue data: {}\n", res_dis);
    }
}