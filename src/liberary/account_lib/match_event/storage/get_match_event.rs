use sqlx::*;
use chrono::{DateTime, Utc};

use crate::liberary::account_lib::match_event::match_event::*;
use crate::liberary::util::functions::build_query::*;

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    id: i64,
    event_id: Option<String>,
    season: String,
    status_code: i16,
    challenger_id: String,
    opponent_id: String,
    start_timestamp: DateTime<Utc>,
}

pub async fn get_match_event(account_id_1: String, account_id_2: String, timestamp: u64, season_name: String, pool: PgPool) -> Result<MatchEvent, Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/match_event/storage/queries/get_match_event.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(account_id_1),
        ArgumentType::String(account_id_2),
        ArgumentType::Timestamptz(DateTime::from_timestamp(timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap())),
        ArgumentType::String(season_name),
    ])?;

    let row = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_one(&pool)
    .await?;

    let match_event = MatchEvent {
        id: Some(row.id as i32),
        event_id: row.event_id,
        season: row.season,
        challenger_id: row.challenger_id.parse()?,
        opponent_id: row.opponent_id.parse()?,
        start_timestamp: row.start_timestamp.timestamp() as u64,
        status: MatchStatus::from_status_code(row.status_code)?,
    };

    Ok(match_event)
}