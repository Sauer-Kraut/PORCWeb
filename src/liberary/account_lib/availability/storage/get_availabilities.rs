use sqlx::*;
use chrono::{DateTime, Utc};

use crate::liberary::util::functions::build_query::*;
use crate::liberary::account_lib::availability::availability::*;


#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,

    repetition_type_code: i32,

    config_mon: bool,
    config_tue: bool,
    config_wed: bool,
    config_thu: bool,
    config_fri: bool,
    config_sat: bool,
    config_sun: bool

}

pub async fn get_availabilities(account_id: u64, pool: PgPool) -> Result<Vec<Availability>, Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/availability/storage/queries/get_availability.sql";
    let query = build_query(query_path, vec![
        ArgumentType::Int(account_id as i64),
    ])?;

    let rows = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_all(&pool)
    .await?;

    let mut availabilities = Vec::new();

    for row in rows {
        let availability = Availability {
            start_timestamp: row.start_date.timestamp() as u64,
            end_timestamp: row.end_date.timestamp() as u64,
            repetition: Repetition::from_type_code(row.repetition_type_code as i16)?,
            repetition_config: match Repetition::from_type_code(row.repetition_type_code as i16)? {
                Repetition::Daily => Some(DailyRepetitionConfig {
                    monday: row.config_mon,
                    tuesday: row.config_tue,
                    wednesday: row.config_wed,
                    thursday: row.config_thu,
                    friday: row.config_fri,
                    saturday: row.config_sat,
                    sunday: row.config_sun,
                }),
                _ => None
            },
        };
        availabilities.push(availability);
    }

    println!("{:?}", availabilities);

    Ok(availabilities)
}