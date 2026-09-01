use sqlx::*;

use crate::liberary::{account_lib::account::user_data::radar_chart::RadarChart, util::functions::build_query::*};


// does not store the schedule beyond the schedule note
pub async fn store_radar_chart(id: String, radar_data: RadarChart, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/account/storage/queries/store_radar_chart.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(id),
        ArgumentType::Float(radar_data.values.iter().find(|(name, _)| name.to_lowercase() == "mobility").map(|(_, value)| *value).unwrap_or(0.0).into()),
        ArgumentType::Float(radar_data.values.iter().find(|(name, _)| name.to_lowercase() == "weight").map(|(_, value)| *value).unwrap_or(0.0).into()),
        ArgumentType::Float(radar_data.values.iter().find(|(name, _)| name.to_lowercase() == "aggressiveness").map(|(_, value)| *value).unwrap_or(0.0).into()),
        ArgumentType::Float(radar_data.values.iter().find(|(name, _)| name.to_lowercase() == "range").map(|(_, value)| *value).unwrap_or(0.0).into()),
        ArgumentType::Float(radar_data.values.iter().find(|(name, _)| name.to_lowercase() == "reactivity").map(|(_, value)| *value).unwrap_or(0.0).into()),
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    // println!("account stored successfully");

    Ok(())
}

