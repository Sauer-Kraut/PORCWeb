use sqlx::*;

use crate::liberary::account_lib::availability::storage::delete_availability::delete_availability;
use crate::liberary::account_lib::availability::storage::get_availabilities::get_availabilities;
use crate::liberary::account_lib::availability::storage::store_availability::store_availability;
use crate::liberary::account_lib::availability::availability::*;

pub async fn update_availabilities(account_id: u64, availabilities: Vec<Availability>, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    
    let stored_availabilities = get_availabilities(account_id, pool.clone()).await?;
    let availabilities_to_delete = stored_availabilities.iter().filter(|stored| !availabilities.iter().any(|new| new.start_timestamp == stored.start_timestamp && new.end_timestamp == stored.end_timestamp && new.repetition.to_type_code() == stored.repetition.to_type_code())).collect::<Vec<_>>();
    let availabilities_to_add = availabilities.iter().filter(|new| !stored_availabilities.iter().any(|stored| new.start_timestamp == stored.start_timestamp && new.end_timestamp == stored.end_timestamp && new.repetition.to_type_code() == stored.repetition.to_type_code())).collect::<Vec<_>>();

    for availability in availabilities_to_delete.iter() {
        delete_availability(account_id, (*availability).clone(), pool.clone()).await?;
    }

    for availability in availabilities_to_add.iter() {
        store_availability(account_id, (*availability).clone(), pool.clone()).await?;
    }

    println!("availabilities updated successfully with {} additions and {} deletions", availabilities_to_add.len(), availabilities_to_delete.len());

    Ok(())
}