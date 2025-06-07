use crate::liberary::dialogue_lib::bot_error::BotError;

pub fn get_user_tag(user_id: String) -> Result<String, BotError> {
    // Assuming `user_id` is a valid Discord user ID
    let user_id = user_id.parse::<u64>()?;
    
    // Simulating fetching user tag from a Discord API or database
    // In a real implementation, you would use the Discord API client to fetch the user
    let user_tag = format!("User#{}", user_id); // Placeholder for actual user tag retrieval logic
    
    Ok(user_tag)
}