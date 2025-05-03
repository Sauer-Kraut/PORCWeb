use sqlx::*;
use chrono::{DateTime, Utc};

use crate::liberary::account_lib::match_event::match_event::*;
use crate::liberary::util::functions::build_query::*;

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    id: i32,
    event_id: Option<i64>,
    season: String,
    status_code: i16,
    challenger_id: i64,
    opponent_id: i64,
    start_timestamp: DateTime<Utc>,
}

pub async fn get_match_event(account_id_1: u64, account_id_2: u64, timestamp: u64, season_name: String, pool: PgPool) -> Result<MatchEvent, Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/match_event/storage/queries/get_match_event.sql";
    let query = build_query(query_path, vec![
        ArgumentType::Int(account_id_1 as i64),
        ArgumentType::Int(account_id_2 as i64),
        ArgumentType::Timestamptz(DateTime::from_timestamp(timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap())),
        ArgumentType::String(season_name),
    ])?;

    let row = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_one(&pool)
    .await?;

    let match_event = MatchEvent {
        id: Some(row.id),
        event_id: row.event_id.map(|id| id as u64),
        season: row.season,
        challenger_id: row.challenger_id as u64,
        opponent_id: row.opponent_id as u64,
        start_timestamp: row.start_timestamp.timestamp() as u64,
        status: MatchStatus::from_status_code(row.status_code)?,
    };

    Ok(match_event)
}