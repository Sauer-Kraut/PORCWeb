use crate::porcbot::config;
use config::*;
use serenity::all::UserId;

use super::dm_send::send_dm;
use super::add_reaction::add_reaction;

pub async fn send_prompt_dm(user_id: u64, prompt: String) -> Result<(), String> {
    let message_id = send_dm(user_id, prompt).await?;

    match UserId::new(user_id as u64).create_dm_channel(get_http()).await {
        Ok(dm_channel) => {
            let allowed_reactions = vec!(ACCEPT_EMOJI, DECLINE_EMOJI);
            for reaction in allowed_reactions.iter() {
                let _ = add_reaction(dm_channel.id.get(), message_id as u64, &reaction).await?;
            }
        },
        Err(err) => return Err(format!("couldnt create dm channel with userId {user_id:?} with error: {err:?}"))
    }
    Ok(())
}