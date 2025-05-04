use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::*;

use crate::liberary::{dialogue_lib::{dialogue_builder::dialogue_builder::DialogueBuilder, dialogue_plan::dialogue_data::DialogueData}, util::functions::build_query::{build_query, ArgumentType}};



#[derive(sqlx::FromRow, Serialize, Deserialize)]
#[derive(Debug)]
struct QueryStruct {
    id: i32,
    created_at: DateTime<Utc>,
    user_id: String,
    dialogue_data: Value,
    error: Option<String>,
    index: i64,
}

pub async fn get_dialogues(limit: u16, min_age: u64, pool: PgPool) -> Result<Vec<DialogueBuilder>, Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/dialogue_lib/dialogue_builder/storage/queries/get_dialogues.sql";
    let query = build_query(query_path, vec![
        ArgumentType::Int(limit as i64),
        ArgumentType::Timestamptz(Utc::now() - chrono::Duration::seconds(min_age as i64)),
    ])?;

    let rows = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_all(&pool)
    .await?;

    let mut dialogue_builders = Vec::new();

    for row in rows {
        let builder = DialogueBuilder {
            dialogue_id: Some(row.id as i64),
            dialogue_data: DialogueData {
                user_id: row.user_id.parse()?,
                data: serde_json::from_value(row.dialogue_data)?,
                error: row.error.clone(),
            },
            index: row.index as u64,
        };

        dialogue_builders.push(builder);
    }

    Ok(dialogue_builders)
}