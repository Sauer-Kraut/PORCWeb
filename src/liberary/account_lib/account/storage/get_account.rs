use sqlx::*;

use crate::liberary::{account_lib::{account::{account::Account, discord_user}, schedule::storage::get_schedule::get_schedule}, util::functions::build_query::{build_query, ArgumentType}};



#[derive(sqlx::FromRow)]
#[derive(Debug)]
struct QueryStruct {
    id: i64,
    username: String,
    discriminator: i32,
    avatar: Option<String>,
    email: Option<String>,
    schedule_note: Option<String>
}

pub async fn get_account(account_id: u64, pool: PgPool) -> Result<Account, Box<dyn std::error::Error>> {
    let query_path = "src/liberary/account_lib/account/storage/queries/get_account.sql";
    let query = build_query(query_path, vec![
        ArgumentType::Int(account_id as i64),
    ])?;

    let row = sqlx::query_as::<Postgres, QueryStruct>(&query)
    .fetch_one(&pool)
    .await?;

    let discord_user = discord_user::DiscordUser {
        id: row.id as u64,
        username: row.username,
        discriminator: row.discriminator as i8,
        avatar: row.avatar,
        email: row.email,
    };

    let schedule = get_schedule(account_id, pool.clone()).await?;

    let account = Account {
        user_info: discord_user,
        schedule: Some(schedule),
    };

    println!("{:?}", account);

    Ok(account)
}