use crate::{liberary::dialogue_lib::bot_error::BotError, porcbot::config};
use config::*;
use serenity::all::UserId;

pub async fn send_dm(user_id: String, content: String) -> Result<u64, BotError> {
    let user_id = UserId::new(user_id.parse().map_err(|_| format!("couldnt parse userId {user_id:?}"))?);

    let dm_channel = user_id.create_dm_channel(get_http()).await?;
    let message = dm_channel.say(get_http(), content).await?;
    Ok(message.id.get() as u64)
}