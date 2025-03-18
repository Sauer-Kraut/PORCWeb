use crate::porcbot::config;
use config::*;
use serenity::all::{UserId, Message, GetMessages};

// list goes [most recent -> least recent]
pub async fn get_dms(user_id: u64) -> Result<Vec<Message>, String> {

    match UserId::new(user_id as u64).create_dm_channel(get_http()).await {
        Ok(dm_channel) => {
            
            match dm_channel.messages(get_http(), GetMessages::new().limit(20)).await {
                Ok(messages) => return Ok(messages),
                Err(err) => return Err(format!("couldnt get dms for userId {user_id:?} with error: {err:?}")),
            }
        },
        Err(err) => return Err(format!("couldnt create dm channel with userId {user_id:?} with error: {err:?}"))
    }
}