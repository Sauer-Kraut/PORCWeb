use sqlx::*;

use crate::liberary::{account_lib::account::pub_account_info::PubAccountInfo, util::functions::build_query::*};


// does not store the schedule beyond the schedule note
pub async fn store_pub_account(account: PubAccountInfo, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/account/storage/queries/store_pub_account.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(account.id.to_string()),
        ArgumentType::String(account.username),
        match account.avatar {
            Some(ref avatar) => ArgumentType::String(avatar.clone()),
            None => ArgumentType::Null,
        },
        match account.schedule {
            Some(ref schedule) => ArgumentType::String(schedule.note.clone()),
            None => ArgumentType::Null,
        },
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    println!("account stored successfully");

    Ok(())
}

