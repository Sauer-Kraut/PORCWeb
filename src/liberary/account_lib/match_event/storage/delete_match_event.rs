use sqlx::*;
use chrono::DateTime;

use crate::liberary::{account_lib::match_event::match_event::MatchEvent, util::functions::build_query::*};

pub async fn delete_match_event(match_event: MatchEvent, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/match_event/storage/queries/delete_match_event.sql";
    let query = build_query(query_path, vec![
        ArgumentType::Timestamptz(DateTime::from_timestamp(match_event.start_timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap())),
        ArgumentType::String(match_event.challenger_id),
        ArgumentType::String(match_event.opponent_id),
        ArgumentType::String(match_event.season),
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    // println!("match event deleted successfully");

    Ok(())
}