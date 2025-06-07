use colored::Colorize;
use serenity::all::UserId;

use crate::{liberary::dialogue_lib::bot_error::BotError, porcbot::util::dm_send_with_attachment::send_dm_with_attachment, AppState};



pub async fn get_season_blueprint(appstate: &AppState, user_id: UserId) -> Result<(), BotError> {

    println!("{}", "Received command to get season blueprint".magenta());

    let pool = appstate.pool.clone();
    let current_season = appstate.season.read().await.clone();

    let blueprint = current_season.clone()
        .ok_or("No current season found".to_string())?
        .get_blueprint(pool.clone()).await?;

    let value = serde_json::to_value(blueprint)
        .map_err(|err| format!("Failed to serialize blueprint: {}", err).to_string())?;

    let message = format!("Here is the blueprint for the next season based on the current season {}", current_season.unwrap().name);

    let _res = send_dm_with_attachment(user_id.to_string(), message, value, "MatchPlanBlueprint.json").await?;
    return Ok(())
}