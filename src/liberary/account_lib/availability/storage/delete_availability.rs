use sqlx::*;
use chrono::DateTime;

use crate::liberary::util::functions::build_query::*;
use crate::liberary::account_lib::availability::availability::*;

pub async fn delete_availability(account_id: String, availability: Availability, pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/account_lib/availability/storage/queries/delete_availability.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(account_id),
        ArgumentType::Timestamptz(DateTime::from_timestamp(availability.start_timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap())),
        ArgumentType::Timestamptz(DateTime::from_timestamp(availability.end_timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap())),
        ArgumentType::Int(availability.repetition.to_type_code() as i64)
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    // println!("availability stored successfully");

    Ok(())
}