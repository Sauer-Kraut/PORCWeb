use serenity::all::UserId;

use crate::liberary::dialogue_lib::bot_error::BotError;

pub fn parse_ds_user_id(unparsed_id: &str) -> Result<UserId, BotError> {
    return Ok(UserId::new(unparsed_id.parse().map_err(|err| format!("couldnt parse user Id {err:?}"))?))
}