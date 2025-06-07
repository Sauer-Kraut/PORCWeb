use crate::{liberary::dialogue_lib::bot_error::BotError, porcbot::config};
use config::*;
use serenity::all::{MessageId, ChannelId, ReactionType};

pub async fn add_reaction(channel_id: u64, message_id: u64, emoji: &str) -> Result<(), BotError> {
    let channel_id = ChannelId::new(channel_id);
    let message_id = MessageId::new(message_id);
    
    let reaction = ReactionType::Unicode(emoji.to_string());

    return Ok(channel_id.create_reaction(get_http(), message_id, reaction).await?);
}