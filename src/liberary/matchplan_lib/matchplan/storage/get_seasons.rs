use chrono::{DateTime, Utc};
use sqlx::*;

use crate::liberary::{account_lib::signup::signup::SignUpInfo, util::functions::build_query::{build_query, ArgumentType}};



#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    season_name: String,
}

// retrieves all seasons ordered by recency
pub async fn get_seasons(pool: PgPool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let query_path = "src/liberary/matchplan_lib/matchplan/storage/queries/get_seasons.sql";
    let query = build_query(query_path, vec![
    ])?;

    let rows = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_all(&pool)
    .await?;

    let mut season_names = Vec::new();

    for row in rows {
        let name = row.season_name.clone();
        season_names.push(name);
    }

    Ok(season_names)
}