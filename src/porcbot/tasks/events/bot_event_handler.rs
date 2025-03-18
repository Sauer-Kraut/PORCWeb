use serenity::{all::{EventHandler, Message}, async_trait};
use colored::Colorize;

use crate::AppState;
use super::{on_message::on_message, on_ready::*};

pub struct BotEventHandler {
    pub appstate: AppState
}

#[async_trait]
impl EventHandler for BotEventHandler {

    async fn ready(&self, ctx: Context, ready: Ready) {
        on_ready(self, ctx, ready).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // println!("{}", "");
        on_message(self, ctx, msg).await;
    }
}