use sqlx::*;

use crate::liberary::{account_lib::account::account_customisation::AccountCustomisation, util::functions::build_query::*};


// does not store the schedule beyond the schedule note
pub async fn store_account_customisation(id: String, cust: AccountCustomisation, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/account/storage/queries/store_account_cust.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(id),
        match cust.region {
            Some(region) => ArgumentType::String(region),
            None => ArgumentType::Null,
        },
        ArgumentType::Int(cust.bp as i64),
        match cust.shiftstones.0 {
            Some(stone) => ArgumentType::String(stone.to_string()),
            None => ArgumentType::Null,
        },
        match cust.shiftstones.1 {
            Some(stone) => ArgumentType::String(stone.to_string()),
            None => ArgumentType::Null,
        },
        match cust.badges.get(0) {
            Some(badge) => ArgumentType::Int(*badge as i64),
            None => ArgumentType::Null,
        },
        match cust.badges.get(1) {
            Some(badge) => ArgumentType::Int(*badge as i64),
            None => ArgumentType::Null,
        },
        match cust.badges.get(2) {
            Some(badge) => ArgumentType::Int(*badge as i64),
            None => ArgumentType::Null,
        }
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    // println!("account stored successfully");

    Ok(())
}

