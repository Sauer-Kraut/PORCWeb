use crate::liberary::dialogue_lib::bot_error::BotError;

use super::dms_get::get_dms;

pub async fn check_response(user_id: u64, prompt: String) -> Result<Option<String>, BotError> {
    let messages = get_dms(user_id).await?;
    for (index, message) in messages.iter().enumerate() {
        if message.content == prompt && message.author.id.get() != user_id {
            
            let mut prompt_index = index as i32;

            while prompt_index >= 0 {
                let index_message =  messages.get(prompt_index as usize).unwrap(); // should be safe since it had to iterate to the start index first
                if index_message.author.id.get() == user_id {
                    return Ok(Some(index_message.content.clone()))
                }
                prompt_index = prompt_index - 1;
            }
        }
    }
    return Ok(None)
}