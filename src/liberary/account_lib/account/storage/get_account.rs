use sqlx::*;

use crate::liberary::{account_lib::{account::{account::Account, discord_user, storage::get_account_full::get_account_full}, schedule::storage::get_schedule::get_schedule}, util::functions::build_query::{ArgumentType, build_query}};



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
    // let query_path = "src/liberary/account_lib/account/storage/queries/get_account.sql";
    // let query = build_query(query_path, vec![
    //     ArgumentType::String(account_id.clone()),
    // ])?;

    // let row = sqlx::query_as::<Postgres, QueryStruct>(&query)
    // .fetch_one(&pool)
    // .await?;

    // let discord_user = discord_user::DiscordUser {
    //     id: row.id.parse()?,
    //     username: row.username,
    //     discriminator: row.discriminator.to_string(),
    //     avatar: row.avatar,
    //     email: row.email,
    // };

    // let account = Account {
    //     user_info: discord_user,
    //     schedule: None,
    // };

    let mut account = get_account_full(account_id, pool).await?.0;
    account.schedule = None; // Remove schedule for the sake of privacy, but lowkey this function might need to be brought out back

    // println!("{:?}", account);

    Ok(account)
}