use colored::Colorize;

use crate::{liberary::dialogue_lib::{bot_error::BotError, dialogue_builder::storage::{get_dialogues::get_dialogues, store_dialogue::store_dialogue}}, AppState};

// extremly inefficient (TODO for the love of god, please fix this)
pub async fn check_dialogues(appstate: &AppState) -> Result<(), BotError> {
    
    let dialogues = match get_dialogues(10000, 120, appstate.pool.clone()).await {
        Ok(dialogues) => dialogues,
        Err(e) => return Err(format!("Error getting dialogues: {}", e).into())
    };

    for _i in 0..(dialogues.len()) {

        let mut plan = match get_dialogues(1, 120, appstate.pool.clone()).await {
            Ok(dialogues) => {
                if let Some(d) = dialogues.first() {
                    d.clone().build().await?
                } else {
                    continue;
                }
            },
            Err(e) => return Err(format!("Error getting dialogues: {}", e).into())
        };

        if let Err(e) = plan.check(appstate).await {
            println!("{}\n{}{}", "An error occured while checking a dialogue: ".red(), e.to_string().bright_red(), " - dialogue check was therefore skiped".yellow());
            match store_dialogue(plan.get_builder(), appstate.pool.clone()).await {
                Ok(_) => {},
                Err(e) => {
                    return Err(format!("Error storing dialogue: {}", e).into())
                }
            };
        } else {
            match store_dialogue(plan.get_builder(), appstate.pool.clone()).await {
                Ok(_) => {},
                Err(e) => {
                    return Err(format!("Error storing dialogue: {}", e).into())
                }
            };
        }
    }
    
    Ok(())
}