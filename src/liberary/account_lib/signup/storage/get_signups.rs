use chrono::{DateTime, Utc};
use sqlx::*;

use crate::liberary::{account_lib::signup::signup::SignUpInfo, util::functions::build_query::{build_query, ArgumentType}};



#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    username: String,
    account_id: String,
    created_at: DateTime<Utc>,
    region: String,
    bp: i64,
}

pub async fn get_signups(lower_date_bound: u64, upper_date_bound: Option<u64>, pool: PgPool) -> Result<Vec<SignUpInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/account_lib/signup/storage/queries/get_signups.sql";
    let query = build_query(query_path, vec![
        ArgumentType::Timestamptz(DateTime::from_timestamp(lower_date_bound as i64, 0).unwrap_or(Utc::now())),
        ArgumentType::Timestamptz(DateTime::from_timestamp(upper_date_bound.unwrap_or(Utc::now().timestamp().try_into()?).try_into()?, 0).unwrap_or(Utc::now())),
    ])?;

    let rows = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_all(&pool)
    .await?;

    let mut signups = Vec::new();

    for row in rows {
        let signup = SignUpInfo {
            discord_id: row.account_id.parse()?,
            username: row.username,
            region: row.region,
            bp: row.bp as u32,
            date: row.created_at.timestamp() as u64,
        };

        signups.push(signup);
    }

    Ok(signups)
}