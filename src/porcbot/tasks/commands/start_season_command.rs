use colored::Colorize;
use serenity::all::Message;
use serde_json;

use crate::{liberary::{dialogue_lib::bot_error::BotError, matchplan_lib::{matchplan::storage::start_season::start_season, matchplan_blueprint::matchplan_blueprint::PlanBlueprint}}, porcbot::util::get_message_attachment::get_message_attachment, AppState};



pub async fn start_season_command(appstate: &AppState, msg: &Message) -> Result<(), BotError> {

    println!("{}", "Received command to start new season".magenta());

    let pool = appstate.pool.clone();

    let attachment_bytes = get_message_attachment(msg, ".json").await?;
    if let None = attachment_bytes.first() {
        return Err("no attachment found".to_string().into())
    }

    let blueprint = serde_json::from_slice::<PlanBlueprint>(attachment_bytes.first().unwrap())
        .map_err(|e| format!("failed to deserialize blueprint: {e}"))?;

    start_season(blueprint, pool).await.map_err(|e| e.to_string())?;
    Ok(())
}