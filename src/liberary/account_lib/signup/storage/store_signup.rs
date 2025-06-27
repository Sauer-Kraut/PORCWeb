use sqlx::*;

use crate::liberary::{account_lib::signup::signup::SignUpInfo, util::functions::build_query::*};


pub async fn store_signup(signup: SignUpInfo, pool: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/account_lib/signup/storage/queries/store_signup.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(signup.discord_id.to_string()),
        ArgumentType::String(signup.region),
        ArgumentType::Int(signup.bp as i64),
    ])?;

    let _res = sqlx::query(&query)
    .execute(&pool)
    .await?;

    // println!("signup stored successfully");

    Ok(())
}