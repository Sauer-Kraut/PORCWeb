use crate::porcbot::config::{get_http, COMMAND_PREFIX};
use crate::porcbot::tasks::commands::match_requests_catch_up::match_request_catch_up;
use crate::porcbot::tasks::functions::has_role::{has_role, has_role_from_message};

pub use super::bot_event_handler::BotEventHandler;
pub use serenity::all::Message;
pub use serenity::prelude::*;

pub async fn on_message(me: &BotEventHandler, ctx: Context, msg: Message) {

    let prefix = COMMAND_PREFIX;
    let striped_message = match msg.content.trim().strip_prefix(prefix) {
        Some(m) => m,
        _ => ""
    };

    match striped_message {
        "match_request_catch_up" => {
            if has_role_from_message(&ctx, &msg, "DEV").await {
                match match_request_catch_up(&me.appstate).await {
                    Ok(_) => {let _ = msg.channel_id.say(get_http(), "match requests caught up successfully!").await;},
                    Err(err) => {let _ = msg.channel_id.say(get_http(), format!("Oh no! An error occurred while executing the command: {err}")).await;}
                }
            } else {
                let _ = msg.channel_id.say(get_http(), format!("You do not have the permision to call this command")).await;
            }
        },
        _ => ()
    }
}