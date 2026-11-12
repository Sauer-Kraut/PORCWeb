use sqlx::*;
use chrono::DateTime;

use crate::liberary::util::functions::build_query::*;
use crate::liberary::account_lib::availability_exception::availability_exception::*;

pub async fn store_availability_exceptions(account_id: String, exc: AvailabilityException, pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/account_lib/availability_exception/storage/queries/store_availability_exception.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(account_id),
        ArgumentType::Timestamptz(DateTime::from_timestamp(exc.start_timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap())),
        ArgumentType::Timestamptz(DateTime::from_timestamp(exc.end_timestamp as i64, 0).unwrap_or(DateTime::from_timestamp(0, 0).unwrap()))
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    Ok(())
}