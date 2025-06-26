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

use crate::backend::storage_lib::StorageMod;

pub static CLIENT: Lazy<Arc<Mutex<Option<Client>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
pub static BOT_TOKEN: Lazy<Arc<String>> = Lazy::new(|| {
    dotenv().ok();
    let t = match StorageMod::read_config().unwrap().dev {
        true => env::var("DISCORD_DEV_BOT_TOKEN").expect("DISCORD_DEV_BOT_TOKEN is not set"), // dev bot token
        false => env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is not set"), // production bot token
    };
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

pub const SERVER_ID: Lazy<Arc<u64>> = Lazy::new( || {
    match StorageMod::read_config().unwrap().dev {
        true => Arc::new(1369654478810382509), // dev server id
        false => Arc::new(1264474928095297536), // main server id
    }
});

pub const STAGE_IDS: Lazy<Arc<(u64, u64, u64)>> = Lazy::new( || {
    match StorageMod::read_config().unwrap().dev {
        true => Arc::new((1369657942256914482, 1369658004391071854, 1369658082312851587)), // dev server stage id
        false => Arc::new((1280230793041674291, 1280231416918970389, 1280231175549489263)), // main server stage ids
    }
});

pub fn get_http() -> Arc<Http> {
    HTTP.clone()
}

// Server roles have to have exactly the same names as the ranks in the matchplan
pub const RANKS: [&'static str; 13] = [
    "Meteorite", "Malachite", "Adamantium", "Mithril", "Platinum", 
    "Diamond", "Gold", "Silver", "Bronze", "Steel", "Copper", 
    "Iron", "Stone"
];