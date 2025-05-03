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

pub async fn get_match_events(account_id: String, pool: PgPool) -> Result<Vec<MatchEvent>, Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/account_lib/match_event/storage/queries/get_match_events.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(account_id),
    ])?;

    let rows = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_all(&pool)
    .await?;

    let mut match_events = Vec::new();

    for row in rows{
        let match_event = MatchEvent {
            id: Some(row.id as i32),
            event_id: row.event_id,
            season: row.season,
            challenger_id: row.challenger_id.parse()?,
            opponent_id: row.opponent_id.parse()?,
            start_timestamp: row.start_timestamp.timestamp() as u64,
            status: MatchStatus::from_status_code(row.status_code)?,
        };
        match_events.push(match_event);
    }

    Ok(match_events)
}