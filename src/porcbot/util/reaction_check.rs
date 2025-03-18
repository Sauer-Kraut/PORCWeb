use crate::porcbot::config;
use config::*;
use serenity::all::{UserId, ReactionType};

use super::dms_get::get_dms;

pub async fn check_reaction(user_id: u64, prompt: String) -> Result<Option<bool>, String> {
    let messages = get_dms(user_id).await?;
    for message in messages.iter() {
        if message.content == prompt && message.author.id.get() != user_id {
            
            let channel = UserId::new(user_id).create_dm_channel(get_http()).await;
            match channel {
                Ok(user_channel) => {

                    let accepted = match user_channel.id.reaction_users(get_http(), message.id, ReactionType::Unicode(ACCEPT_EMOJI.to_string()), Some(50), None).await {
                        Ok(users) => {users.iter().any(|user| user.id.get() == user_id)}
                        Err(err) => {return Err(format!("error occured on user id {user_id} while trying to get reaction users: {err:?}"));}
                    };

                    let declined = match user_channel.id.reaction_users(get_http(), message.id, ReactionType::Unicode(DECLINE_EMOJI.to_string()), Some(50), None).await {
                        Ok(users) => {users.iter().any(|user| user.id.get() == user_id)}
                        Err(err) => {return Err(format!("error occured on user id {user_id} while trying to get reaction users: {err:?}"));}
                    };

                    if accepted && declined {
                        return Ok(None)
                    } else if accepted {
                        return Ok(Some(true))
                    } else if declined {
                        return Ok(Some(false))
                    }
                },
                Err(err) => return Err(format!("error occured on user id {user_id} while trying to get channel: {err:?}")),
            }

        }
    }
    return Ok(None)
}