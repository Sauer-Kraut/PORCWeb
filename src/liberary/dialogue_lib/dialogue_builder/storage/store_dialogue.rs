use sqlx::*;

use crate::liberary::{dialogue_lib::dialogue_builder::dialogue_builder::DialogueBuilder, util::functions::build_query::*};


pub async fn store_dialogue(builder: DialogueBuilder, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {

    match builder.dialogue_id {
        Some(dialogue_id) => {

            let query_path = "src/liberary/dialogue_lib/dialogue_builder/storage/queries/update_dialogue_builder.sql";
            let query = build_query(query_path, vec![
                ArgumentType::Int(dialogue_id as i64),
                ArgumentType::String(builder.dialogue_data.user_id),
                ArgumentType::JSONB(serde_json::to_value(builder.dialogue_data.data)?),
                match builder.dialogue_data.error {
                    Some(ref error) => ArgumentType::String(error.clone()),
                    None => ArgumentType::Null,
                },
                ArgumentType::Int(builder.index as i64)
            ])?;

            let _res = sqlx::query(&query)
            .execute(&pool)
            .await?;
        },

        None => {

            let query_path = "src/liberary/dialogue_lib/dialogue_builder/storage/queries/store_dialogue_builder.sql";
            let query = build_query(query_path, vec![
                ArgumentType::String(builder.dialogue_data.user_id),
                ArgumentType::JSONB(serde_json::to_value(builder.dialogue_data.data)?),
                match builder.dialogue_data.error {
                    Some(ref error) => ArgumentType::String(error.clone()),
                    None => ArgumentType::Null,
                },
                ArgumentType::Int(builder.index as i64)
            ])?;

            let _res = sqlx::query(&query)
            .execute(&pool)
            .await?;
        },
    }

    println!("dialogue stored successfully");

    Ok(())
}