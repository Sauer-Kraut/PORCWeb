use sqlx::*;

use crate::liberary::{matchplan_lib::matchplan_blueprint::matchplan_blueprint::PlanBlueprint, util::functions::build_query::{self, build_query}};

pub async fn start_season(blueprint: PlanBlueprint, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "src/liberary/matchplan_lib/matchplan/storage/queries/start_season.sql";
    let query = build_query(query_path, vec![
        build_query::ArgumentType::JSONB(serde_json::to_value(blueprint)?)
    ])?;

    let _result = sqlx::query(&query)
    .execute(&pool)
    .await?;

    Ok(())
}