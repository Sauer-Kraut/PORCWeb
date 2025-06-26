use colored::Colorize;
use serenity::all::{GuildId, RoleId};

use crate::{liberary::dialogue_lib::{bot_error::BotError, dialogue_builder::storage::{get_dialogues::get_dialogues, store_dialogue::store_dialogue}}, porcbot::config::{get_http, RANKS, SERVER_ID}, AppState};
use crate::liberary::matchplan_lib::matchplan::matchplan::MatchPlan;

pub async fn update_ranking_roles(appstate: &AppState, matchplan: MatchPlan) -> Result<(), BotError> {

    let guild_id = GuildId::new(SERVER_ID.as_ref().clone());

    let rank_role_names = RANKS.iter().map(|f| f.to_string()).collect::<Vec<String>>();
    let guild_roles = guild_id.roles(get_http()).await?;

    let mut rank_roles = Vec::new();
    for role_name in rank_role_names.iter() {
        if let Some(role) = guild_roles.iter().find(|(_, r)| r.name == *role_name) {
            rank_roles.push(role);
        } else {
            return Err(format!("Role '{}' not found on the server", role_name).into());
        }
    }

    let rank_roles_ids = rank_roles.iter().map(|r| *r.0).collect::<Vec<RoleId>>();

    let members = guild_id.members(get_http(), None, None).await?;

    let mut role_remover_tasks = Vec::new();

    for member in members.iter() {
        
        for role in &member.roles {
            if rank_roles.iter().any(|r| *r.0 == *role) {

                role_remover_tasks.push(async {
                    let res = member.remove_roles(get_http(), &rank_roles_ids).await;
                    res
                })
            }
        }
    }

    // errors get logged and ignored
    let remover_taks_results = futures::future::join_all(role_remover_tasks).await;
    let remover_tasks_errs = remover_taks_results.iter().filter(|res| res.is_err()).collect::<Vec<_>>();
    for err in remover_tasks_errs {
        if let Err(e) = err {
            println!("{}\n{}{}", "An error occurred while removing roles: ".red(), e.to_string().bright_red(), " - role removal was therefore skipped".yellow());
        }
    }



    let mut add_role_tasks = Vec::new();

    for member in members.iter().filter(|m| matchplan.players.iter().any(|p| p.id == m.user.id.to_string())) {
        
        add_role_tasks.push(async {
            let division = match matchplan.divisions.iter().find(|d| d.players.iter().any(|p| p.id == member.user.id.to_string())){
                Some(d) => d.name.clone(),
                None => return Err(format!("Player {} not found in matchplan", member.user.name).into()),
            };

            let role = match rank_roles.iter().find(|r| r.1.name == division) {
                Some(r) => r.0,
                None => return Err(format!("Role for division '{}' not found", division).into()),
            };

            let r: Result<_, BotError> = member.add_role(get_http(), role).await.map_err(|e| e.into());
            r
        })
    }


    // errors get logged and ignored
    let add_taks_results = futures::future::join_all(add_role_tasks).await;
    let add_tasks_errs = add_taks_results.iter().filter(|res| res.is_err()).collect::<Vec<_>>();
    for err in add_tasks_errs {
        if let Err(e) = err {
            println!("{}\n{}{}", "An error occurred while removing roles: ".red(), e.to_string().bright_red(), " - role removal was therefore skipped".yellow());
        }
    }
    
    Ok(())
}