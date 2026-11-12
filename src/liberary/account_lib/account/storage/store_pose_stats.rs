use sqlx::*;

use crate::liberary::{util::functions::build_query::*};


// does not store the schedule beyond the schedule note
pub async fn store_pose_stats(id: String, stats: Option<Vec<(String, f32)>>, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/account/storage/queries/store_pose_stats.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(id),
        ArgumentType::JSONB(serde_json::value::to_value(stats)?),
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    // println!("account stored successfully");

    Ok(())
}

