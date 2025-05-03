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

pub async fn get_match_events_from_ids(match_event_ids: Vec<i32>, pool: PgPool) -> Result<Vec<MatchEvent>, Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account/storage/queries/retrieve/get_match_event_from_id.sql";

    let mut match_events = Vec::new();

    for match_event_id in match_event_ids {
        let query = build_query(query_path, vec![
            ArgumentType::Int(match_event_id as i64),
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
        match_events.push(match_event);
    }

    Ok(match_events)
}