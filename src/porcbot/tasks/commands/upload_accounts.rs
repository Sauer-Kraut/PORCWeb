use colored::Colorize;
use crate::liberary::account_lib::availability::storage::update_availabilities;
use crate::liberary::account_lib::schedule::schedule::Schedule;
use crate::liberary::dialogue_lib::bot_error::BotError;
use crate::{liberary::account_lib::account::storage::store_account::store_account, AppState};
use crate::backend::storage_lib::StorageMod;
use crate::liberary::account_lib::availability::storage::update_availabilities::update_availabilities;


// Command to go upload all accounts in the userdate folder accounts json file to the databank
// IMPORTANT: will overwrite any newer account changes in the databank!
pub async fn upload_accounts(appstate: &AppState) -> Result<(), BotError> {

    println!("{}", "Received command to catch up with match requests".magenta());

    let accounts_to_upload = StorageMod::read_accounts()
        .map_err(|e| format!("Error while getting accounts: {}", e))?;

    println!("{}", format!("{}{}", "Found ".green(), accounts_to_upload.len().to_string().bright_white()));

    for (_key, account) in accounts_to_upload.iter() {
        match store_account(account.clone(), appstate.pool.clone()).await {
            Ok(_) => {
                update_availabilities(account.user_info.id.clone(), account.schedule.clone().unwrap_or(Schedule {availabilities: vec!(), matches: vec!(), note: "".to_owned()}).availabilities.clone(), appstate.pool.clone()).await.unwrap_or(());
                println!("{}", format!("{}{}{}","Account ".green(), account.user_info.id.clone().bright_white(), " uploaded successfully".green()))
            },
            Err(e) => {
                println!("{}", format!("{}{}\n{}", "Error uploading account ".red(), account.user_info.id.bright_white(), e.to_string().bright_red()));
                return Err(format!("Error uploading account: {}", e.to_string()).into())
            }
        }
    }
    Ok(())
}