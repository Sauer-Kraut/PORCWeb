use crate::porcbot::config;
use config::*;
use serenity::all::{MessageId, ChannelId, ReactionType};

pub async fn add_reaction(channel_id: u64, message_id: u64, emoji: &str) -> Result<(), String> {
    let channel_id = ChannelId::new(channel_id);
    let message_id = MessageId::new(message_id);
    
    let reaction = ReactionType::Unicode(emoji.to_string());

    if let Err(err) = channel_id.create_reaction(get_http(), message_id, reaction).await {
        return Err(err.to_string())
    } else {
        Ok(())
    }
}