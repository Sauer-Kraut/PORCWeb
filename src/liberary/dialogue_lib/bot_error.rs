use serde::{Deserialize, Serialize};
use thiserror::Error;
use serenity::Error as SerenityError;




#[derive(Error, Debug)]
#[derive(Serialize, Deserialize)]
pub enum BotError {

    #[error("Discord API error: {0}")]
    APIError(String),

    #[error("error: {0}")]
    LogicError(String),

    #[error("DB error: {0}")]
    DBError(String),
}

impl From<String> for BotError
{
    fn from(value: String) -> Self {
        Self::LogicError(value.into())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for BotError
{
    fn from(value: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::LogicError(value.to_string())
    }
}

impl From<sqlx::Error> for BotError
{
    fn from(value: sqlx::Error) -> Self {
        Self::DBError(value.to_string())
    }
}

impl From<SerenityError> for BotError
{
    fn from(value: SerenityError) -> Self {
        Self::APIError(value.to_string())
    }
}
