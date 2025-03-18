use crate::porcbot::util::response_check::check_response;

pub use super::bot_event_handler::BotEventHandler;
use colored::Colorize;
pub use serenity::prelude::*;
pub use serenity::model::gateway::Ready;

pub async fn on_ready(me: &BotEventHandler, _ctx: Context, _ready: Ready) {
    println!("{} \n\n", "Bot has launched".bright_white());
}