use actix_http::cookie;
use actix_web::cookie::time::error;
use async_std::sync::Mutex;
use actix_web::{FromRequest, HttpRequest};
use actix_web::{web, HttpResponse, Responder, cookie::Cookie};
use colored::Colorize;
use tokio;
use std::collections::HashMap;
use std::sync::{mpsc, Arc};
use serde::{Deserialize, Serialize};
use crate::{sanetize_username, AppState, Division, GetRequestPlanPackage, GetRequestSignUpPackage, Player, SignUpInfo, StorageMod};
use async_std::fs;
use reqwest::Client;
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiscordAuthenticationQueryParam {
    pub code: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenRequestParam {
    pub client_id: String,
    pub client_secret: String
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: String,
    scope: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PutRequestLoggedInRecvPackage {
    pub title: String,
    pub id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PutRequestLoggedInSendPackage {
    pub title: String,
    pub data: Option<DiscordUser>,
    pub error: Option<String>
}


pub async fn discord_callback(appstate: web::Data<AppState>, query: web::Query<DiscordAuthenticationQueryParam>) -> impl Responder {
    println!("\nYay, we received a discord callback with query: \n{:?}!", query);

    let client = Client::new();
    let clinet_info = StorageMod::read_secrets().unwrap();
    
    let access_token = match exchang_code_for_token(&query.code, clinet_info).await {
        Ok(token) => token,
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.to_string().red().bold()); 
            return HttpResponse::InternalServerError().body("")
        },
    };

    let response = client
        .get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await;

    let unwraped_response = match response {
        Ok(res) => res,
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.to_string().red().bold()); 
            return HttpResponse::InternalServerError().body("")
        },
    };

    let result = match unwraped_response.json::<DiscordUser>().await {
        Ok(res) => res,
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.to_string().red().bold()); 
            return HttpResponse::InternalServerError().body("")
        },
    };

    println!("Discord User: {:?}", result);

    let mut locked_logins = appstate.logins.lock().await;
    let mut logins = locked_logins.clone();

    let session_id = generate_session_id().await;

    logins.insert(session_id.clone(), result);
    *locked_logins = logins.clone();
    match StorageMod::save_logins(logins, &appstate.logins_path) {
        Ok(_) => {},
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.to_string().red().bold()); 
            return HttpResponse::InternalServerError().body("")
        }
    };

    let cookie = Cookie::build("browser_id", &session_id[..])
        .domain("porc.mywire.org")         // TODO: needs to be updated for deployment
        .path("/")
        .http_only(false)
        .secure(true)               
        .same_site(actix_web::cookie::SameSite::None)       // TODO: needs to be set to strict for deployment
        .finish();

    return HttpResponse::Ok()
        .cookie(cookie)
        .body(fs::read_to_string("PORC-Front/dist/index.html").await.unwrap())
}



async fn exchang_code_for_token(code: &str, info: TokenRequestParam) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let id = info.client_id;
    let secret = info.client_secret;

    let request_params = [
        ("client_id", &id[..]),
        ("client_secret", &secret[..]),
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", "https://porc.mywire.org/discord/callback")
    ];

    let response = client
        .post("https://discord.com/api/oauth2/token")
        .form(&request_params)
        .send()
        .await?;

    let token_response = response.json::<TokenResponse>().await?;


    Ok(token_response.access_token)
}



pub async fn put_logged_in(info: web::Json<PutRequestLoggedInRecvPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received PUT Request for logged in status".bold().cyan());

    let (data_sender, data_receiver) = mpsc::channel();
    let (error_sender, error_receiver) = mpsc::channel();

    let logins_clone = appstate.logins.clone();
    let client_id = info.id.clone();

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {
        
        match logins_clone.as_ref().lock().await.get(&client_id) {
            Some(entry) => {
                data_sender.send(entry.clone()).unwrap()
            },
            None => {error_sender.send("no login found for client id".to_string()).unwrap()}
        };

    }).await.unwrap();

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    let data = match data_receiver.recv() {
        Ok(data) => Some(data),
        Err(_) => None,
    };

    HttpResponse::Ok().json(PutRequestLoggedInSendPackage {
        title: "Server Logged in response".to_string(),
        data,
        error
    })
}



async fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}