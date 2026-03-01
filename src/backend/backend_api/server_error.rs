use actix_web::{HttpResponse, ResponseError};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::liberary::{dialogue_lib::bot_error::BotError, util::functions::build_query::QueryBuildError};





#[derive(Error, Debug)]
#[derive(Serialize, Deserialize)]
pub enum ServerError {

    #[error("DB error: {0}")]
    DBError(String),

    #[error("Invalid Input: {0}")]
    BadInput(String),

    #[error("Unauthorized request")]
    Unauthorized,

    #[error("Discord bot error: {0}")]
    BotError(#[from] BotError),

    #[error("Query building error: {0}")]
    QueryBuildError(#[from] QueryBuildError),

    #[error("error: {0}")]
    Other(String)
}

impl ResponseError for ServerError {

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {

        eprintln!("{} {}", "An error occured:".red(), self.to_string().red().bold());

        match self {
            ServerError::DBError(error) => HttpResponse::InternalServerError().body(error.to_string()),
            ServerError::BadInput(error) => HttpResponse::BadRequest().body(error.to_string()),
            ServerError::Unauthorized => HttpResponse::Unauthorized().finish(),
            ServerError::BotError(error) => HttpResponse::InternalServerError().body(error.to_string()),
            ServerError::QueryBuildError(error) => HttpResponse::InternalServerError().body(error.to_string()),
            ServerError::Other(error) => HttpResponse::InternalServerError().body(error.to_string()),
        }
    }
}


impl From<Box<dyn std::error::Error + Send + Sync>> for ServerError
{
    fn from(value: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for ServerError
{
    fn from(value: Box<dyn std::error::Error>) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<String> for ServerError
{
    fn from(value: String) -> Self {
        Self::Other(value.into())
    }
}

impl From<&str> for ServerError
{
    fn from(value: &str) -> Self {
        Self::Other(value.to_string().into())
    }
}

impl From<sqlx::Error> for ServerError
{
    fn from(value: sqlx::Error) -> Self {
        Self::DBError(value.to_string())
    }
}