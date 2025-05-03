use sqlx::*;

use crate::liberary::{matchplan_lib::matchplan_match::matchplan_match::Match, util::functions::build_query::{self, build_query}};

pub async fn update_match(match_info: Match, season_name: String, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "src/liberary/matchplan_lib/matchplan_match/storage/queries/store_match.sql";
    let query = build_query(query_path, vec![
        build_query::ArgumentType::String(match_info.p1.id.to_string()),
        build_query::ArgumentType::String(match_info.p2.id.to_string()),
        build_query::ArgumentType::Int(match_info.p1score.unwrap_or(0) as i64),
        build_query::ArgumentType::Int(match_info.p2score.unwrap_or(0) as i64),
        build_query::ArgumentType::String(season_name)
    ])?;

    let _result = sqlx::query(&query)
    .execute(&pool)
    .await?;

    Ok(())
}