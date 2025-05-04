use sqlx::*;

use crate::liberary::{account_lib::{account::{account::Account, discord_user}, schedule::storage::get_schedule::get_schedule}, util::functions::build_query::{build_query, ArgumentType}};



#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    id: String,
    username: String,
    discriminator: i32,
    avatar: Option<String>,
    email: Option<String>,
    schedule_note: Option<String>
}

pub async fn get_account(account_id: String, pool: PgPool) -> Result<Account, Box<dyn std::error::Error + Send + Sync>> {
    let query_path = "src/liberary/account_lib/account/storage/queries/get_account.sql";
    let query = build_query(query_path, vec![
        ArgumentType::String(account_id.clone()),
    ])?;

    let row = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_one(&pool)
    .await?;

    let discord_user = discord_user::DiscordUser {
        id: row.id.parse()?,
        username: row.username,
        discriminator: row.discriminator.to_string(),
        avatar: row.avatar,
        email: row.email,
    };

    let schedule = get_schedule(account_id, pool.clone()).await?;

    let account = Account {
        user_info: discord_user,
        schedule: Some(schedule),
    };

    // println!("{:?}", account);

    Ok(account)
}