use sqlx::*;

use crate::liberary::{account_lib::{availability::storage::get_availabilities::get_availabilities, match_event::storage::get_match_events::get_match_events, schedule::schedule::Schedule}, util::functions::build_query::*};



#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    schedule_note: Option<String>,
}

pub async fn get_schedule(account_id: u64, pool: PgPool) -> Result<Schedule, Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account/storage/queries/retrieve/get_schedule_note.sql";
    let query = build_query(query_path, vec![
        ArgumentType::Int(account_id as i64),
    ])?;

    let note = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_one(&pool)
    .await?;

    let availabilities = get_availabilities(account_id, pool.clone()).await?;
    let match_events = get_match_events(account_id, pool.clone()).await?;

    let schedule = Schedule {
        availabilities,
        matches: match_events.iter().filter_map(|match_event| if let Some(id) = match_event.id {Some(id)} else {None}).collect(),
        note: note.schedule_note.unwrap_or("".to_string()),
    };

    println!("{:?}", schedule);

    Ok(schedule)
}