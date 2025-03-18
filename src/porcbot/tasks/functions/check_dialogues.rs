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
                plan.check(appstate).await?;
                if plan.index == 600 {
                    builders_lock.remove(index);
                } else {
                    *builder = plan.get_builder();
                }
            },
            None => (),
        }

        drop(builders_lock);
        appstate.refresh().await; // safes after every dialogue to avoid losing data on error or crash, especially since the discord API always takes a bit so the function takes a few seconds
    }
    Ok(())
}