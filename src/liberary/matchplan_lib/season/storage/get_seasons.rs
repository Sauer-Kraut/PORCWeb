use chrono::{DateTime, Utc};
use sqlx::*;

use crate::liberary::{matchplan_lib::season::season::Season, util::functions::build_query::{build_query}};



#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    season_name: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    pause_end_date: DateTime<Utc>
}

// retrieves all seasons ordered by recency
pub async fn get_seasons(pool: PgPool) -> Result<Vec<Season>, Box<dyn std::error::Error>> {
    let query_path = "src/liberary/matchplan_lib/season/storage/queries/get_seasons.sql";
    let query = build_query(query_path, vec![
    ])?;

    let rows = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_all(&pool)
    .await?;

    let mut season_names = Vec::new();

    for row in rows {
        let season = Season {
            name: row.season_name.clone(),
            start_timestamp: row.start_date.timestamp() as u64,
            end_timestamp: row.end_date.timestamp() as u64,
            pause_end_timestamp: row.pause_end_date.timestamp() as u64,
        };
        season_names.push(season);
    }

    Ok(season_names)
}