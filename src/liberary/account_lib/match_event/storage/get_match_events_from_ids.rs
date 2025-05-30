use futures::future::join_all;
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

pub async fn get_match_events_from_ids(match_event_ids: Vec<i32>, pool: PgPool) -> Result<Vec<MatchEvent>, Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/match_event/storage/queries/get_match_event_from_id.sql";

    let mut match_events = Vec::new();
    let mut row_handles = vec!();

    for match_event_id in match_event_ids {
        let query = build_query(query_path, vec![
            ArgumentType::Int(match_event_id as i64),
        ])?;
        // Move the query String into the async block so it lives long enough
        row_handles.push(async {
            let owned_query = query;
            sqlx::query_as::<Postgres, QueryStruct>(&owned_query)
                .fetch_one(&pool)
                .await
        });
    }
    let row_res = join_all(row_handles).await;

    for r in row_res {
        let row = r?;

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