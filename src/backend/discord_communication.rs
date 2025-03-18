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
use async_std::fs;
use reqwest::Client;
use uuid::Uuid;
use actix_web::cookie::time::OffsetDateTime;
use std::time::{SystemTime, Duration};

use crate::AppState;
use crate::backend::account_lib::DiscordUser;

use super::account_lib::PubAccountInfo;
use super::storage_lib::StorageMod;
use super::account_lib::{Account, Schedule};



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

#[derive(Debug, Deserialize, Serialize)]
pub struct PutRequestLoggedInRecvPackage {
    pub title: String,
    pub id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PutRequestLoggedInSendPackage {
    pub title: String,
    pub data: Option<PubAccountInfo>,
    pub error: Option<String>
}


pub async fn discord_callback(appstate: web::Data<AppState>, query: web::Query<DiscordAuthenticationQueryParam>) -> impl Responder {
    println!("\nYay, we received a discord callback with query: \n{:?}!", query);

    let client = Client::new();
    let clinet_info = StorageMod::read_secrets().unwrap();
    
    let access_token = match exchang_code_for_token(&query.code, clinet_info, appstate.config.url.clone()).await {
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

    logins.insert(session_id.clone(), result.id.clone());
    *locked_logins = logins.clone();
    match StorageMod::save_logins(logins) {
        Ok(_) => {},
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.to_string().red().bold()); 
            return HttpResponse::InternalServerError().body("")
        }
    };

    let mut locked_accounts = appstate.accounts.lock().await;
    let mut accounts = locked_accounts.clone();

    if !accounts.contains_key(&result.id) {
        
        let new_account = Account {
            user_info: result.clone(),
            schedule: Some(Schedule {
                availabilities: vec!(),
                matches: vec!(),
                notes: "".to_string(),
            })
        };

        accounts.insert(result.id, new_account);
        *locked_accounts = accounts.clone();
        match StorageMod::save_accounts(accounts) {
            Ok(_) => {},
            Err(err) => {
                println!("{} {}", "An Error occured:".red().bold(), err.to_string().red().bold()); 
                return HttpResponse::InternalServerError().body("")
            }
        };
    }

    
    let expiry = OffsetDateTime::now_utc() + Duration::from_secs(60 * 60 * 24 * 30);

    let cookie = Cookie::build("browser_id", &session_id[..])
        .domain(appstate.config.domain.clone())         // TODO: needs to be updated for deployment
        .path("/")
        .http_only(false)
        .secure(true)               
        .expires(expiry)
        .same_site(actix_web::cookie::SameSite::None)       // TODO: needs to be set to strict for deployment
        .finish();

    return HttpResponse::Ok()
        .cookie(cookie)
        .body(fs::read_to_string("PORC-Front/dist/index.html").await.unwrap())
}



async fn exchang_code_for_token(code: &str, info: TokenRequestParam, url: String) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let id = info.client_id;
    let secret = info.client_secret;

    let request_params = [
        ("client_id", &id[..]),
        ("client_secret", &secret[..]),
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", &format!("{}{}", url, "/discord/callback"))
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

    let accounts_clone = appstate.accounts.clone();
    let accounts_lock = accounts_clone.lock().await;
    let accounts_clone = accounts_lock.clone();

    drop(accounts_lock);

    // We spawn an asyncronus thread in order to be able to handle many requests at once
    println!("startig async thread");
    tokio::task::spawn(async move {
        
        match logins_clone.as_ref().lock().await.clone().get(&client_id) {
            Some(entry) => {
                let discord_id = entry.clone();
                match accounts_clone.get(&discord_id) {
                    Some(value) => {let _ = data_sender.send(value.clone());}
                    None => {error_sender.send("No account for discord id found".to_string()).unwrap();}
                }
            },
            None => {error_sender.send("no login found for client id".to_string()).unwrap()}
        };

    }).await.unwrap();

    let error = match error_receiver.try_recv(){
        Ok(err) => {println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); Some(err)},
        Err(_) => None,
    };

    let data = match data_receiver.recv() {
        Ok(data) => Some(data.get_pub_info()),
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