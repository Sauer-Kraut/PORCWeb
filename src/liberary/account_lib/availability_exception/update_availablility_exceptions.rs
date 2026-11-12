use futures::future::join_all;
use sqlx::*;

use crate::liberary::account_lib::availability_exception::availability_exception::AvailabilityException;
use crate::liberary::account_lib::availability_exception::storage::delete_availability_exception::delete_availability_exception;
use crate::liberary::account_lib::availability_exception::storage::get_availabilit_exceptions::get_availability_exceptions;
use crate::liberary::account_lib::availability_exception::storage::store_availability_exception::store_availability_exceptions;

pub async fn update_availability_exceptions(account_id: String, exceps: Vec<AvailabilityException>, pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let stored_exceptions = get_availability_exceptions(account_id.clone(), pool.clone()).await?;
    let exceptions_to_delete = stored_exceptions.iter().filter(|stored| !exceps.iter().any(|new| new.start_timestamp == stored.start_timestamp && new.end_timestamp == stored.end_timestamp)).collect::<Vec<_>>();
    let exceptions_to_add = exceps.iter().filter(|new| !stored_exceptions.iter().any(|stored| new.start_timestamp == stored.start_timestamp && new.end_timestamp == stored.end_timestamp)).collect::<Vec<_>>();

    let mut delete_handles = Vec::new();

    for exc in exceptions_to_delete.iter() {
        delete_handles.push(delete_availability_exception(account_id.clone(), (*exc).clone(), pool.clone()));
    }

    let exceptions_to_delete_res: Result<Vec<()>, Box<dyn std::error::Error + Send + Sync>> = join_all(delete_handles).await.into_iter().collect();
    exceptions_to_delete_res?;

    let mut add_handles = Vec::new();

    for exc in exceptions_to_add.iter() {
        add_handles.push(store_availability_exceptions(account_id.clone(), (*exc).clone(), pool.clone()));
    }

    let exceptions_to_add_res: Result<Vec<()>, Box<dyn std::error::Error + Send + Sync>> = join_all(add_handles).await.into_iter().collect();
    exceptions_to_add_res?;

    println!("availability exceptions updated successfully with {} additions and {} deletions", exceptions_to_add.len(), exceptions_to_delete.len());

    Ok(())
}