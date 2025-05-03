use crate::porcbot::config;
use config::*;
use serenity::all::UserId;

pub async fn send_dm(user_id: String, content: String) -> Result<u64, String> {
    let user_id = UserId::new(user_id.parse().map_err(|_| format!("couldnt parse userId {user_id:?}"))?);

    match user_id.create_dm_channel(get_http()).await {
        Ok(dm_channel) => {

            match dm_channel.say(get_http(), content).await {
                Ok(message) => {
                    Ok(message.id.get() as u64)
                },
                Err(err) => Err(format!("couldnt send message to userId {user_id:?} with error: {err:?}")),
            }
        },
        Err(err) => return Err(format!("couldnt create dm channel with userId {user_id:?} with error: {err:?}"))
    }
}