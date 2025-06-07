use crate::{liberary::dialogue_lib::bot_error::BotError, porcbot::config};
use config::*;
use serenity::all::{CreateAttachment, CreateMessage, UserId};

pub async fn send_dm_with_attachment(user_id: String, content: String, attachment: serde_json::Value, attachment_name: &str) -> Result<u64, BotError> {
    let user_id = UserId::new(user_id.parse().map_err(|_| format!("couldnt parse userId {user_id:?}"))?);

    let dm_channel = user_id.create_dm_channel(get_http()).await?;
    let attachment_bytes = serde_json::to_string_pretty(&attachment).map_err(|e| e.to_string())?.into_bytes();
    let create_attachment = CreateAttachment::bytes(attachment_bytes, attachment_name);

    let message = dm_channel.send_message(get_http(), CreateMessage::new().content(content).add_file(create_attachment)).await?;
    Ok(message.id.get() as u64)
}