use crate::porcbot::config;
use config::*;
use serenity::all::{CreateAttachment, CreateMessage, UserId};

pub async fn send_dm_with_attachment(user_id: String, content: String, attachment: serde_json::Value, attachment_name: &str) -> Result<u64, String> {
    let user_id = UserId::new(user_id.parse().map_err(|_| format!("couldnt parse userId {user_id:?}"))?);

    match user_id.create_dm_channel(get_http()).await {
        Ok(dm_channel) => {

            let attachment_bytes = serde_json::to_string_pretty(&attachment).map_err(|e| e.to_string())?.into_bytes();
            let create_attachment = CreateAttachment::bytes(attachment_bytes, attachment_name);

            match dm_channel.send_message(get_http(), CreateMessage::new().content(content).add_file(create_attachment)).await {
                Ok(message) => {
                    Ok(message.id.get() as u64)
                },
                Err(err) => Err(format!("couldnt send message to userId {user_id:?} with error: {err:?}")),
            }
        },
        Err(err) => return Err(format!("couldnt create dm channel with userId {user_id:?} with error: {err:?}"))
    }
}