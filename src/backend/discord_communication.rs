use actix_web::{web, HttpResponse, Responder, cookie::Cookie};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use async_std::fs;
use reqwest::Client;
use uuid::Uuid;
use actix_web::cookie::time::OffsetDateTime;
use std::time::Duration;

use crate::backend::storage_lib::StorageMod;
use crate::liberary::account_lib::account::account::Account;
use crate::liberary::account_lib::account::discord_user::DiscordUser;
use crate::liberary::account_lib::account::pub_account_info::PubAccountInfo;
use crate::liberary::account_lib::account::storage::get_account::get_account;
use crate::liberary::account_lib::account::storage::store_account::store_account;
use crate::liberary::account_lib::login::login::LogIn;
use crate::liberary::account_lib::login::storage::get_login;
use crate::liberary::account_lib::login::storage::store_login::store_login;
use crate::liberary::account_lib::schedule::schedule::Schedule;
use crate::AppState;



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

    let session_id = generate_session_id().await;

    let login = LogIn {
        key: session_id.clone(),
        account_id: result.id.clone(),
        creation_timestamp: 0,
    };

    match store_login(login, appstate.pool.clone()).await {
        Ok(_) => {},
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.to_string().red().bold()); 
            return HttpResponse::InternalServerError().body("")
        }
    };


    match get_account(result.id.clone(), appstate.pool.clone()).await {
        Ok(_) => {},
        Err(_) => {
            let new_account = Account {
                user_info: result.clone(),
                schedule: Some(Schedule {
                    availabilities: vec!(),
                    matches: vec!(),
                    note: "".to_string(),
                })
            };
            match store_account(new_account, appstate.pool.clone()).await {
                Ok(_) => {},
                Err(err) => {
                    println!("{} {}", "An Error occured:".red().bold(), err.to_string().red().bold()); 
                    return HttpResponse::InternalServerError().body("")
                }
            };
        }
    };

    
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



// Request to receive the account belonging to the provided account id
pub async fn put_logged_in(info: web::Json<PutRequestLoggedInRecvPackage>, appstate: web::Data<AppState>) -> impl Responder {
    println!("\n{}", "Received PUT Request for logged in status".bold().cyan());

    let result: Result<PubAccountInfo, String> = 'scope: {

        let login = match get_login::get_login(info.id.clone(), appstate.pool.clone()).await {
            Ok(login) => login,
            Err(err) => {
                break 'scope Err(format!("Error while getting login: {:?}", err));
            }
        };

        let account = get_account(login.account_id.clone(), appstate.pool.clone()).await;
        match account {
            Ok(account) => {
                let pub_account_info = account.get_pub_info();
                break 'scope Ok(pub_account_info);
            },
            Err(err) => {
                break 'scope Err(format!("Error while getting account: {:?}", err));
            }
        }
    };

    match result {
        Ok(account) => {
            return HttpResponse::Ok().json(PutRequestLoggedInSendPackage {
                title: "Account Info response".to_string(),
                data: Some(account),
                error: None
            })
        },
        Err(err) => {
            println!("{} {}", "An Error occured:".red().bold(), err.red().bold()); 
            return HttpResponse::InternalServerError().json(PutRequestLoggedInSendPackage {
                title: "Account Info response".to_string(),
                data: None,
                error: Some(err)
            })
        }
    }
}



async fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}