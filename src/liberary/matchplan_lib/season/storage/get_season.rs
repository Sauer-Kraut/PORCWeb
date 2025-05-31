use sqlx::*;
use chrono::{DateTime, Utc};

use crate::liberary::matchplan_lib::season::season::Season;
use crate::liberary::util::functions::build_query::*;

#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    season_name: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    pause_end_date: DateTime<Utc>
}

pub async fn get_season(season_name: String, pool: PgPool) -> Result<Option<Season>, Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/matchplan_lib/season/storage/queries/get_season.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(season_name),
    ])?;

    let row = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_one(&pool)
    .await;

    let row = match row {
        Ok(v) => v,
        Err(err) => {

            match err {
                Error::RowNotFound => return Ok(None),
                _ => return Err(Box::new(err)),
            }
        },
    };

    let season = Season {
        name: row.season_name.clone(),
        start_timestamp: row.start_date.timestamp() as u64,
        end_timestamp: row.end_date.timestamp() as u64,
        pause_end_timestamp: row.pause_end_date.timestamp() as u64,
    };

    Ok(Some(season))
}