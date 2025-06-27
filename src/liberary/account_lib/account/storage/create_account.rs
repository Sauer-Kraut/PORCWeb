use sqlx::*;

use crate::liberary::{account_lib::account::account::Account, util::functions::build_query::*};


// does not store the schedule beyond the schedule note
// does nothing if an account already exists
pub async fn create_account(account: Account, pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/account_lib/account/storage/queries/create_account.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(account.user_info.id.to_string()),
        ArgumentType::String(account.user_info.username),
        ArgumentType::Int(account.user_info.discriminator.parse()?),
        match account.user_info.avatar {
            Some(ref avatar) => ArgumentType::String(avatar.clone()),
            None => ArgumentType::Null,
        },
        match account.user_info.email {
            Some(ref email) => ArgumentType::String(email.clone()),
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

    // println!("account stored successfully");

    Ok(())
}