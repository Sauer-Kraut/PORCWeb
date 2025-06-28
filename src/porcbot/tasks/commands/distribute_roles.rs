use colored::Colorize;
use serenity::all::UserId;

use crate::{liberary::{dialogue_lib::bot_error::BotError, matchplan_lib::matchplan::storage::matchplan_get::get_matchplan}, porcbot::{tasks::functions::update_ranking_roles::update_ranking_roles, util::dm_send_with_attachment::send_dm_with_attachment}, AppState};



pub async fn dist_roles(appstate: &AppState) -> Result<(), BotError> {

    println!("{}", "Received command to distribute roles".magenta());

    
    let season_opt = appstate.season.read().await.clone();
    let season = match season_opt {
        Some(season) => season.name.clone(),
        None => {
            return Err(BotError::LogicError("No current season found".to_owned().into()));
        }
    };

    let matchplan = get_matchplan(season, appstate.pool.clone()).await?;

    let _res = update_ranking_roles(appstate, matchplan).await?;

    return Ok(())
}