use sqlx::*;
use chrono::DateTime;

use crate::liberary::account_lib::match_event::match_event::*;
use crate::liberary::util::functions::build_query::*;

pub async fn store_match_event(match_event: MatchEvent, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "";
    let query = build_query(query_path, vec![
        ArgumentType::Timestamptz(DateTime::from_timestamp(match_event.start_timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap())),
        ArgumentType::Int(match_event.challenger_id as i64),
        ArgumentType::Int(match_event.opponent_id as i64),
        ArgumentType::String(match_event.season),
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    println!("match event stored successfully");

    Ok(())
}

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// TODO: add event id to the query
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!