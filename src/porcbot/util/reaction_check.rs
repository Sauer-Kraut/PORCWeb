use crate::{liberary::dialogue_lib::bot_error::BotError, porcbot::config};
use config::*;
use serenity::all::{UserId, ReactionType};

use super::dms_get::get_dms;

pub async fn check_reaction(user_id: u64, prompt: String) -> Result<Option<bool>, BotError> {
    let messages = get_dms(user_id).await?;
    for message in messages.iter() {
        if message.content == prompt && message.author.id.get() != user_id {
            
            let user_channel = UserId::new(user_id).create_dm_channel(get_http()).await?;

            let accepted = user_channel.id.reaction_users(get_http(), message.id, ReactionType::Unicode(ACCEPT_EMOJI.to_string()), Some(50), None).await?
                .iter().any(|user| user.id.get() == user_id);

            let declined = user_channel.id.reaction_users(get_http(), message.id, ReactionType::Unicode(DECLINE_EMOJI.to_string()), Some(50), None).await? 
                .iter().any(|user| user.id.get() == user_id);

            if accepted && declined {
                return Ok(None)
            } else if accepted {
                return Ok(Some(true))
            } else if declined {
                return Ok(Some(false))
            }

        }
    }
    return Ok(None)
}