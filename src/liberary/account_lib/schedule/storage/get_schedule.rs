use sqlx::*;

use crate::liberary::{account_lib::{availability::storage::get_availabilities::get_availabilities, match_event::storage::get_match_events::get_match_events, schedule::schedule::Schedule}, util::functions::build_query::*};



#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    schedule_note: Option<String>,
}

pub async fn get_schedule(account_id: String, pool: PgPool) -> Result<Schedule, Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/account_lib/schedule/storage/queries/get_schedule_note.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(account_id.to_string()),
    ])?;

    let note = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_one(&pool)
    .await?;

    let availabilities = get_availabilities(account_id.clone(), pool.clone()).await?;
    let match_events = get_match_events(account_id, pool.clone()).await?;

    let schedule = Schedule {
        availabilities,
        matches: match_events.iter().filter_map(|match_event| if let Some(id) = match_event.id {Some(id)} else {None}).collect(),
        note: note.schedule_note.unwrap_or("".to_string()),
    };

    // println!("{:?}", schedule);

    Ok(schedule)
}