use sqlx::*;
use chrono::{DateTime, Utc};

use crate::liberary::asset_lib::image::image_entry::ImageEntry;
use crate::liberary::util::functions::build_query::*;

#[derive(sqlx::FromRow)]
struct QueryStruct {
    pub id: uuid::Uuid,
    pub object_name: String,
    pub owner_id: Option<String>,
    pub author: String,
    pub link: String,
    pub filetype: String,
    pub created_at: DateTime<Utc>,
}

impl From<QueryStruct> for ImageEntry {
    fn from(query_struct: QueryStruct) -> Self {
        ImageEntry {
            id: query_struct.id.to_string(),
            object_name: query_struct.object_name,
            owner_id: query_struct.owner_id,
            author: query_struct.author,
            link: query_struct.link,
            filetype: query_struct.filetype,
            created_at: query_struct.created_at.timestamp() as u64,
        }
    }
}

pub async fn get_image_entries_by_id(image_entry_ids: Vec<String>, pool: PgPool) -> Result<Vec<ImageEntry>, Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/asset_lib/image/storage/queries/get_image_entry_by_id.sql";
    let query = build_query(query_path, vec![
        ArgumentType::StringArray(image_entry_ids),
    ])?;

    let rows = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_all(&pool)
    .await?;

    Ok(rows.into_iter().map(|row| ImageEntry::from(row)).collect())
}