use futures::future::join_all;
use sqlx::*;

use crate::liberary::account_lib::availability::storage::delete_availability::delete_availability;
use crate::liberary::account_lib::availability::storage::get_availabilities::get_availabilities;
use crate::liberary::account_lib::availability::storage::store_availability::store_availability;
use crate::liberary::account_lib::availability::availability::*;

pub async fn update_availabilities(account_id: String, availabilities: Vec<Availability>, pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let stored_availabilities = get_availabilities(account_id.clone(), pool.clone()).await?;
    let availabilities_to_delete = stored_availabilities.iter().filter(|stored| !availabilities.iter().any(|new| new.start_timestamp == stored.start_timestamp && new.end_timestamp == stored.end_timestamp && new.repetition.to_type_code() == stored.repetition.to_type_code())).collect::<Vec<_>>();
    let availabilities_to_add = availabilities.iter().filter(|new| !stored_availabilities.iter().any(|stored| new.start_timestamp == stored.start_timestamp && new.end_timestamp == stored.end_timestamp && new.repetition.to_type_code() == stored.repetition.to_type_code())).collect::<Vec<_>>();

    let mut delete_handles = Vec::new();

    for availability in availabilities_to_delete.iter() {
        delete_handles.push(delete_availability(account_id.clone(), (*availability).clone(), pool.clone()));
    }

    let availabilities_to_delete_res: Result<Vec<()>, Box<dyn std::error::Error + Send + Sync>> = join_all(delete_handles).await.into_iter().collect();
    availabilities_to_delete_res?;

    let mut add_handles = Vec::new();

    for availability in availabilities_to_add.iter() {
        add_handles.push(store_availability(account_id.clone(), (*availability).clone(), pool.clone()));
    }

    let availabilities_to_add_res: Result<Vec<()>, Box<dyn std::error::Error + Send + Sync>> = join_all(add_handles).await.into_iter().collect();
    availabilities_to_add_res?;

    println!("availabilities updated successfully with {} additions and {} deletions", availabilities_to_add.len(), availabilities_to_delete.len());

    Ok(())
}