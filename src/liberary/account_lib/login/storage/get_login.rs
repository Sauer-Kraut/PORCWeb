use sqlx::*;
use chrono::{DateTime, Utc};

use crate::liberary::{account_lib::login::login::LogIn, util::functions::build_query::*};

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    id: String,
    created_at: DateTime<Utc>,
}

pub async fn get_login(key: String, pool: PgPool) -> Result<LogIn, Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/login/storage/queries/get_login.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(key.clone()),
    ])?;

    let row = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_one(&pool)
    .await?;

    let login = LogIn {
        key: key,
        account_id: row.id.parse()?,
        creation_timestamp: row.created_at.timestamp() as u64,
    };

    // println!("{:?}", login);

    Ok(login)
}