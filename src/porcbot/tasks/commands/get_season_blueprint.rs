use colored::Colorize;
use serenity::all::UserId;

use crate::{porcbot::util::dm_send_with_attachment::send_dm_with_attachment, AppState};



pub async fn get_season_blueprint(appstate: &AppState, user_id: UserId) -> Result<(), String> {

    println!("{}", "Received command to get season blueprint".magenta());

    let pool = appstate.pool.clone();
    let current_season = appstate.season.read().await.clone();

    let blueprint = match current_season.clone() {
        Some(s) => {
            match s.get_blueprint(pool.clone()).await {
                Ok(v) => v,
                Err(e) => return Err(e.to_string())
            }
        },
        None => return Err("no current season could be found".to_string()),
    };

    let value = match serde_json::to_value(blueprint) {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };

    let message = format!("Here is the blueprint for the next season based on the current season {}", current_season.unwrap().name);

    match send_dm_with_attachment(user_id.to_string(), message, value, "MatchPlanBlueprint.json").await {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err),
    };
}