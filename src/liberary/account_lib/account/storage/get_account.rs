use sqlx::*;

use crate::liberary::account_lib::account::{account::Account, storage::get_account_full::get_accounts_full};



pub async fn get_accounts(account_ids: Vec<String>, pool: PgPool) -> Result<Vec<Account>, Box<dyn std::error::Error + Send + Sync>> {
    let accounts_full = get_accounts_full(account_ids, pool).await?;
    let accounts = accounts_full
        .into_iter()
        .map(|(mut account, _)| {
            account.schedule = None;
            account
        })
        .collect();

    Ok(accounts)
}

pub async fn get_account(account_id: String, pool: PgPool) -> Result<Account, Box<dyn std::error::Error + Send + Sync>> {
    get_accounts(vec![account_id], pool)
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| Box::new(sqlx::Error::RowNotFound) as Box<dyn std::error::Error + Send + Sync>)
}