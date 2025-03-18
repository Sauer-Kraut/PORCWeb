use std::env;
use std::sync::Arc;
use once_cell::sync::Lazy;
use serenity::all::Http;
use serenity::{async_trait};
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;
use tokio::time::{interval, Duration};
use serenity::Client;
use dotenvy::dotenv;

pub static CLIENT: Lazy<Arc<Mutex<Option<Client>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
pub static BOT_TOKEN: Lazy<Arc<String>> = Lazy::new(|| {
    dotenv().ok();
    let t = env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is not set");
    Arc::new(t)
}); 
pub static HTTP: Lazy<Arc<Http>> = Lazy::new(|| {
    Arc::new(Http::new(&BOT_TOKEN))
});

pub static INTENTS: Lazy<Arc<GatewayIntents>> = Lazy::new(|| {
    let i = GatewayIntents::GUILD_MESSAGES 
    | GatewayIntents::MESSAGE_CONTENT
    | GatewayIntents::DIRECT_MESSAGE_REACTIONS;
    Arc::new(i)
});

pub const COMMAND_PREFIX: &str = "!";

pub const ACCEPT_EMOJI: &str = "✅";
pub const DECLINE_EMOJI: &str= "❌";

pub const SERVER_ID: u64 = 1264474928095297536;
pub const STAGE_IDS: (u64, u64, u64) = (1280230793041674291, 1280231416918970389, 1280231175549489263);

pub fn get_http() -> Arc<Http> {
    HTTP.clone()
}