use colored::Colorize;

use crate::AppState;

pub async fn check_dialogues(appstate: &AppState) -> Result<(), String> {
    let dialogues_clone = appstate.dialogues.clone();
    let start_lenght = dialogues_clone.lock().await.len();

    for index in 0..start_lenght {

        let mut builders_lock = dialogues_clone.lock().await;
        let entry = builders_lock.get_mut(index);

        match entry {
            Some(builder) => {
                let mut plan = builder.clone().build().await?;
                if plan.index == 600 || plan.index == 400  {
                    builders_lock.remove(index);
                } else {
                    if let Err(e) = plan.check(appstate).await {
                        println!("{}\n{}{}", "An error occured while checking a dialogue:".red(), e.bright_red(), " - dialogue check was therefore skiped".yellow());
                        continue;
                    } else {
                        *builder = plan.get_builder();
                    }
                }
            },
            None => (),
        }

        drop(builders_lock);
        appstate.refresh_dialogues().await; // safes after every dialogue to avoid losing data on error or crash, especially since the discord API always takes a bit so the function takes a few seconds
    }
    Ok(())
}