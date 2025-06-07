use crate::{liberary::dialogue_lib::bot_error::BotError, porcbot::config};
use config::*;
use serenity::all::{UserId, Message, GetMessages};

// list goes [most recent -> least recent]
pub async fn get_dms(user_id: u64) -> Result<Vec<Message>, BotError> {

    let dm_channel = UserId::new(user_id as u64).create_dm_channel(get_http()).await?;

    let messages = dm_channel.messages(get_http(), GetMessages::new().limit(20)).await?;
    return Ok(messages)
}