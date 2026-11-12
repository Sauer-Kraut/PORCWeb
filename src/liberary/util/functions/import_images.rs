use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::liberary::asset_lib::image::image_entry::ImageEntry;
use crate::liberary::asset_lib::image::storage::store_image_entry::store_image_entry;

pub async fn import_images(pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let import_dir = Path::new("resources/images/import");
    let banner_dir = Path::new("resources/images/banner/global");

    if !import_dir.exists() {
        return Err(format!("Import directory does not exist: {}", import_dir.display()).into());
    }

    fs::create_dir_all(banner_dir)?;

    let image_paths = fs::read_dir(import_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();

    for src_path in image_paths {
        let original_name = src_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or("Image filename was not valid UTF-8")?;

        let filetype = src_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .unwrap_or_default();

        if filetype != "png" && filetype != "jpg" && filetype != "jpeg" {
            continue; // Skip non-image files
        }

        let stem = src_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or("Image stem was not valid UTF-8")?;

        let (object_name, author) = parse_image_filename(stem)
            .ok_or_else(|| format!("Filename does not match expected format: {original_name}"))?;

        let image_id = Uuid::new_v4().to_string();
        let extension = format!(".{filetype}");
        let renamed_name = format!("{image_id}{extension}");
        let dest_path = banner_dir.join(&renamed_name);

        fs::rename(&src_path, &dest_path)?;

        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let image_entry = ImageEntry {
            id: image_id,
            object_name,
            owner_id: None,
            author,
            link: format!("banner/{renamed_name}"),
            filetype,
            created_at: created_at as u64,
        };

        store_image_entry(image_entry, pool.clone()).await?;
    }

    Ok(())
}

fn parse_image_filename(stem: &str) -> Option<(String, String)> {
    let (object_name, remainder) = stem.split_once("__by_")?;
    let (author, _) = remainder.rsplit_once("__")?;

    let object_name = object_name.trim().to_string();
    let author = author.trim().to_string();

    if object_name.is_empty() || author.is_empty() {
        return None;
    }

    Some((object_name, author))
}