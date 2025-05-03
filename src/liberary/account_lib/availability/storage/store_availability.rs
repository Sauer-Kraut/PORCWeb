use sqlx::*;
use chrono::DateTime;

use crate::liberary::util::functions::build_query::*;
use crate::liberary::account_lib::availability::availability::*;

pub async fn store_availability(account_id: u64, availability: Availability, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/availability/storage/queries/store_availability.sql";
    let query = build_query(query_path, vec![
        ArgumentType::Int(account_id as i64),
        ArgumentType::Timestamptz(DateTime::from_timestamp(availability.start_timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap())),
        ArgumentType::Timestamptz(DateTime::from_timestamp(availability.end_timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap())),
        ArgumentType::Int(availability.repetition.to_type_code() as i64),
        ArgumentType::Bool(availability.repetition_config.as_ref().map_or(false, |config| config.monday)),
        ArgumentType::Bool(availability.repetition_config.as_ref().map_or(false, |config| config.tuesday)),
        ArgumentType::Bool(availability.repetition_config.as_ref().map_or(false, |config| config.wednesday)),
        ArgumentType::Bool(availability.repetition_config.as_ref().map_or(false, |config| config.thursday)),
        ArgumentType::Bool(availability.repetition_config.as_ref().map_or(false, |config| config.friday)),
        ArgumentType::Bool(availability.repetition_config.as_ref().map_or(false, |config| config.saturday)),
        ArgumentType::Bool(availability.repetition_config.as_ref().map_or(false, |config| config.sunday)),
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    println!("availability stored successfully");

    Ok(())
}