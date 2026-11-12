use sqlx::*;
use chrono::{DateTime, Utc};

use crate::liberary::account_lib::availability_exception::availability_exception::AvailabilityException;
use crate::liberary::util::functions::build_query::*;


#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}

pub async fn get_availability_exceptions(account_id: String, pool: PgPool) -> Result<Vec<AvailabilityException>, Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/account_lib/availability_exception/storage/queries/get_availability_exceptions.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(account_id),
    ])?;

    let rows = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_all(&pool)
    .await?;

    let mut av_excepts = Vec::new();

    for row in rows {
        let except = AvailabilityException {
            start_timestamp: row.start_date.timestamp() as u64,
            end_timestamp: row.end_date.timestamp() as u64
        };
        av_excepts.push(except);
    }

    // println!("{:?}", availabilities);

    Ok(av_excepts)
}