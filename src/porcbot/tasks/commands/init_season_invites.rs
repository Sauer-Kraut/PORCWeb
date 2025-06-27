use actix::fut::future::result;
use colored::Colorize;
use serenity::all::{GuildId, Message};
use serde_json;

use crate::{liberary::{account_lib::signup::storage::get_signups::get_signups, dialogue_lib::{bot_error::BotError, dialogue_initiator::dialogue_initiator::DialogueInitator}, matchplan_lib::{matchplan::storage::{matchplan_get::get_matchplan, start_season::start_season}, matchplan_blueprint::matchplan_blueprint::PlanBlueprint, season::season::Season}}, porcbot::{config::{get_http, SERVER_ID}, tasks::functions::profile_competitors, util::get_message_attachment::get_message_attachment}, AppState};
use crate::porcbot::tasks::functions::profile_competitors::profile_competitors;



pub async fn init_season_invites_command(appstate: &AppState, msg: &Message) -> Result<(), BotError> {

    let attachment_bytes = get_message_attachment(msg, ".json").await?;
    if let None = attachment_bytes.first() {
        return Err("no attachment found".to_string().into())
    }

    let new_season = serde_json::from_slice::<Season>(attachment_bytes.first().unwrap())
        .map_err(|e| format!("failed to deserialize season: {e}"))?;

    println!("{}{}{}{}", "Received command to initiate season invites for season ".magenta(), new_season.name.magenta().bold(), " starting ".magenta(), new_season.start_timestamp.to_string().magenta().bold());

    let _res = profile_competitors(appstate).await?;
    

    let season_opt = appstate.season.read().await.clone();
    let season = match season_opt {
        Some(season) => season,
        None => {
            return Err(BotError::LogicError("No current season found".to_owned().into()));
        }
    };

    let current_participants = get_matchplan(season.name.clone(), appstate.pool.clone()).await?.players
        .iter()
        .map(|p| p.id.clone())
        .collect::<Vec<String>>();


    let guild_id = GuildId::new(SERVER_ID.as_ref().clone());
    let members = guild_id.members(get_http(), None, None).await?;

    let competitor_role = guild_id.roles(get_http()).await?
        .into_iter()
        .find(|(_, r)| r.name == "Competitor")
        .map(|(id, _)| id)
        .ok_or_else(|| BotError::LogicError("Competitor role not found".to_owned().into()))?;

    let competitors = members.iter()
        .filter(|m| m.roles.iter().any(|r| *r == competitor_role))
        .map(|m| m.user.id.to_string())
        .collect::<Vec<String>>();


    let signups = get_signups(season.start_timestamp, None, appstate.pool.clone()).await?.iter()
        .map(|s| s.discord_id.clone())
        .collect::<Vec<String>>();


    
    let to_leap = competitors.iter()
        .filter(|c| current_participants.contains(c) && !signups.contains(c))
        .cloned()
        .collect::<Vec<String>>();

    let to_invite = competitors.iter()
        .filter(|c| !to_leap.contains(c) && !signups.contains(c))
        .cloned()
        .collect::<Vec<String>>();




    let mut leap_tasks = vec!();
    let mut invite_tasks = vec!();

    for user_id in to_leap {

        let season = new_season.clone();
        leap_tasks.push(async move {
            DialogueInitator::initiate_season_leap(appstate, user_id, season).await
        });
    }

    for user_id in to_invite {

        let season = new_season.clone();
        invite_tasks.push(async move {
            DialogueInitator::initiate_season_invite(appstate, user_id, season).await
        });
    }

    // let mut results = futures::future::join_all(leap_tasks).await;
    // let mut invite_results = futures::future::join_all(invite_tasks).await;

    // IMPORTANT!
    // execution is handled syncronously to avoid to many open connections to DB (We are on the peassant plan, at some point I might set up a local DB that comes with its own fun set of issues)

    for task in leap_tasks {
        let res = task.await;

        match res {
            Ok(_) => {},
            Err(err) => {
                println!("{}\n{}", "Failed to initiate dialogue: ".red(), err.to_string().bright_red());
            }
        }
    };

    for task in invite_tasks {
        let res = task.await;

        match res {
            Ok(_) => {},
            Err(err) => {
                println!("{}\n{}", "Failed to initiate dialogue: ".red(), err.to_string().bright_red());
            }
        }
    };



    println!("command completed succesfully! \n");


    Ok(())
}