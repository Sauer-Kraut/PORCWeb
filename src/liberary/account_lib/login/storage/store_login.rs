use sqlx::*;

use crate::liberary::{account_lib::login::login::LogIn, util::functions::build_query::*};

pub async fn store_login(login: LogIn, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/login/storage/queries/store_login.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(login.key),
        ArgumentType::String(login.account_id.to_string()),
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    // println!("login stored successfully");

    Ok(())
}