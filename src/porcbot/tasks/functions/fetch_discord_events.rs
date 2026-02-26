use std::{fmt::format, num::NonZero};

use colored::Colorize;
use serenity::{all::{GuildId, Member, RoleId}, model::guild};

use crate::{AppState, liberary::{dialogue_lib::{bot_error::BotError}, discord_lib::discord_event::discord_event::DiscordEvent}, porcbot::config::{RANKS, SERVER_ID, get_http}};

pub async fn fetch_discord_events(appstate: &AppState) -> Result<Vec<DiscordEvent>, BotError> {

    let guild_id = GuildId::new(SERVER_ID.as_ref().clone());
    let events = guild_id.scheduled_events(get_http(), true).await?;

    let mut discord_events = Vec::new();

    for event in events.iter() {
        let event_struct = DiscordEvent {
            title: event.name.clone(),
            description: event.description.clone().unwrap_or_default(),
            img_id: "".to_string(), // fuck this field
            interested: event.user_count.unwrap_or_default() as u32,
            link: format!("https://discordapp.com/events/{}/{}", SERVER_ID.as_ref().clone(), event.id.to_string()),
            start_time: event.start_time.timestamp() as u64,
            place: "Online".to_string(),
            live: event.status == serenity::all::ScheduledEventStatus::Active,
        };
        discord_events.push(event_struct);
    }
    
    Ok(discord_events)
}