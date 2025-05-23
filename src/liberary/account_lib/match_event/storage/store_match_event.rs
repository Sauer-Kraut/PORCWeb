use sqlx::*;
use chrono::DateTime;

use crate::liberary::account_lib::match_event::match_event::*;
use crate::liberary::util::functions::build_query::*;

pub async fn store_match_event(match_event: MatchEvent, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/match_event/storage/queries/store_match_event.sql";
    let query = build_query(query_path, vec![
        match match_event.event_id {
            Some(id) => ArgumentType::String(id),
            None => ArgumentType::Null,
        },
        ArgumentType::Timestamptz(DateTime::from_timestamp(match_event.start_timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap())),
        ArgumentType::Int(match_event.status.to_type_code() as i64),
        ArgumentType::String(match_event.challenger_id.to_string()),
        ArgumentType::String(match_event.opponent_id.to_string()),
        ArgumentType::String(match_event.season),
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    // println!("match event stored successfully");

    Ok(())
}

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// TODO: add event id to the query
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!