use thiserror::Error;
use serenity::Error as SerenityError;




#[derive(Error, Debug)]
pub enum BotError {

    #[error("Discord API error: {0}")]
    APIError(#[from] SerenityError),

    #[error("error: {0}")]
    LogicError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("DB error: {0}")]
    DBError(#[from] sqlx::Error),
}

impl From<String> for BotError
{
    fn from(value: String) -> Self {
        Self::LogicError(value.into())
    }
}
