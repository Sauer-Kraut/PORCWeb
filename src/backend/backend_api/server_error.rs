use actix_web::{HttpResponse, ResponseError};
use colored::Colorize;
use thiserror::Error;

use crate::liberary::dialogue_lib::bot_error::BotError;





#[derive(Error, Debug)]
pub enum ServerError {

    #[error("DB error: {0}")]
    DBError(#[from] sqlx::Error),

    #[error("Invalid Input: {0}")]
    BadInput(String),

    #[error("Unauthorized request")]
    Unauthorized,

    #[error("Discord bot error: {0}")]
    BotError(#[from] BotError),

    #[error("error: {0}")]
    Other(#[from] Box<dyn std::error::Error>)
}

impl ResponseError for ServerError {

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {

        eprintln!("{} {}", "An error occured:".red(), self.to_string().red().bold());

        match self {
            ServerError::DBError(error) => HttpResponse::InternalServerError().body(error.to_string()),
            ServerError::BadInput(error) => HttpResponse::BadRequest().body(error.to_string()),
            ServerError::Unauthorized => HttpResponse::Unauthorized().finish(),
            ServerError::BotError(error) => HttpResponse::InternalServerError().body(error.to_string()),
            ServerError::Other(error) => HttpResponse::InternalServerError().body(error.to_string()),
        }
    }
}


impl From<Box<dyn std::error::Error + Send + Sync>> for ServerError
{
    fn from(value: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Other(value)
    }
}

impl From<String> for ServerError
{
    fn from(value: String) -> Self {
        Self::Other(value.into())
    }
}