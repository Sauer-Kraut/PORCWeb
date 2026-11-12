use sqlx::*;

use crate::liberary::{asset_lib::image::image_entry::ImageEntry, util::functions::build_query::*};


// does not store the schedule beyond the schedule note
pub async fn store_image_entry(image_entry: ImageEntry, pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/asset_lib/image/storage/queries/store_image_entry.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(image_entry.id.to_string()),
        ArgumentType::String(image_entry.object_name),
        image_entry.owner_id.map(ArgumentType::String).unwrap_or(ArgumentType::Null),
        ArgumentType::String(image_entry.author),
        ArgumentType::String(image_entry.link),
        ArgumentType::String(image_entry.filetype),
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    // println!("image entry stored successfully");

    Ok(())
}