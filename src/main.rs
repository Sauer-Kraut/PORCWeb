pub mod data_lib;
pub mod client_communication;
pub mod storage_lib;
pub mod bot_communication;
pub mod discord_communication;
use discord_communication::{discord_callback, put_logged_in, DiscordUser};
use async_std::sync::Mutex;
use actix_web::FromRequest;
use bot_communication::*;
use futures::future::{ready, Ready};
use data_lib::*;
use storage_lib::*;
use client_communication::*;
use async_std::fs;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::http;
use actix_files::Files;
use colored::Colorize;
use tokio;
use std::collections::HashMap;
use std::sync::{Arc};
use tokio::sync::RwLock;






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

    let matchplan_path = "userdata/SeasonMatchPlan.json".to_string();
    let signups_path = "userdata/SeasonSignUps.json".to_string();
    let logins_path = "userdata/DiscordLogIns.json".to_string();

    println!("read 1");
    let read_plan = StorageMod::read_matchplan(&matchplan_path)?;

    println!("read 2");
    let logins = Arc::new(Mutex::new(StorageMod::read_logins(&logins_path)?));

    // println!("Read Matchplan: {}", read_plan);

    let matchplan = Arc::new(Mutex::new(Some(read_plan)));
    // StorageMod::save_signups(vec!(), "src/Season4SignUps.json")?;
    println!("read 3");
    let signups = Arc::new(Mutex::new(StorageMod::read_signups(&signups_path)?));
    println!("secrets: {:?}", StorageMod::read_secrets().unwrap());

    println!("\n{}\n\n", "Server has launched");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                .allowed_origin("http://localhost:8081") 
                .allowed_origin("http://localhost:5173") 
                .allowed_origin("https://porc.mywire.org") // Update with your frontend's origin
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
            .app_data(web::Data::new(AppState {
                matchplan: matchplan.clone(),
                signups: signups.clone(),
                logins: logins.clone(),
                matchplan_path: matchplan_path.clone(),
                signups_path: signups_path.clone(),
                logins_path: logins_path.clone()
            }))
            .service(web::resource("/").to(index))
            .service(web::resource("/signup").to(index))
            .service(web::resource("/rules").to(index))
            .service(web::resource("/qaa").to(index))
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
            .route(web::post().to(put_logged_in)))
            .service(Files::new("/", "./PORC-Front/dist").index_file("index.html"))

    })
    .bind("[::]:8081")? // Caddy forwarts requests to our URL to the local port 8081
    .run()
    .await
}



pub struct AppState {
    matchplan: Arc<Mutex<Option<MatchPlan>>>,
    signups: Arc<Mutex<Vec<SignUpInfo>>>,
    logins: Arc<Mutex<HashMap<String, DiscordUser>>>,
    matchplan_path: String,
    signups_path: String,
    logins_path: String
}