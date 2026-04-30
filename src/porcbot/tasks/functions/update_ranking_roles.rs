use std::collections::HashMap;

use colored::Colorize;
use serenity::all::{EditMember, GuildId, RoleId};
use futures::stream::{self, StreamExt};

use crate::{liberary::dialogue_lib::{bot_error::BotError, dialogue_builder::storage::{get_dialogues::get_dialogues, store_dialogue::store_dialogue}}, porcbot::config::{get_http, RANKS, SERVER_ID}, AppState};
use crate::liberary::matchplan_lib::matchplan::matchplan::MatchPlan;

pub async fn update_ranking_roles(appstate: &AppState, matchplan: MatchPlan) -> Result<(), BotError> {

    let guild_id = GuildId::new(SERVER_ID.as_ref().clone());

    let mut rank_role_names = RANKS.iter().map(|f| f.to_string()).collect::<Vec<String>>();
    let mut sub_division_role_names = rank_role_names.clone().iter().map(|r| r.to_string() + " I").collect::<Vec<String>>();
    sub_division_role_names.append(&mut rank_role_names.clone().iter().map(|r| r.to_string() + " II").collect::<Vec<String>>());
    sub_division_role_names.append(&mut rank_role_names.clone().iter().map(|r| r.to_string() + " III").collect::<Vec<String>>());

    let guild_roles = guild_id.roles(get_http()).await?;

    let mut rank_roles = Vec::new();
    for role_name in rank_role_names.iter() {
        if let Some(role) = guild_roles.iter().find(|(_, r)| r.name == *role_name) {
            rank_roles.push(role);
        } else {
            return Err(format!("Role '{}' not found on the server", role_name).into());
        }
    }

    for role_name in sub_division_role_names.iter() {
        if let Some(role) = guild_roles.iter().find(|(_, r)| r.name == *role_name) {
            rank_roles.push(role);
        } 
        // Do nothing if not found, not every division has sub divisions
    }

    let rank_roles_ids = rank_roles.iter().map(|r| *r.0).collect::<Vec<RoleId>>();

    let mut members = guild_id.members(get_http(), None, None).await?;


    let mut division_map: HashMap<String, String> = HashMap::new();

    for division in matchplan.divisions.iter() {
        for player in division.players.iter() {
            division_map.insert(player.id.clone(), division.name.clone());
        }
    }

    let role_map: HashMap<String, RoleId> = HashMap::from_iter(rank_roles.iter().map(|(id, role)| (role.name.clone(), **id)));

    let mut role_editor_tasks = Vec::new();

    for member in members.iter_mut() {

        let division = division_map.get(&member.user.id.to_string());

        let mut target_role_ids = member.roles.iter().filter(|r| !rank_roles_ids.contains(*r)).map(|r| *r).collect::<Vec<RoleId>>();

        match division {
            Some(division_unwraped) => {
                let role = match role_map.get(division_unwraped.split(' ').next().unwrap_or(division_unwraped.as_str())) {
                    Some(r) => *r,
                    None => return Err(format!("Role for division '{}' not found", division_unwraped).into()),
                };
                target_role_ids.push(role);

                let sub_role = match role_map.get(division_unwraped) {
                    Some(r) => *r,
                    None => return Err(format!("Role for division '{}' not found", division_unwraped).into()),
                };

                if !target_role_ids.contains(&sub_role) {
                    target_role_ids.push(sub_role);
                }
            },
            None => {},
        };

        let mut target_sorted = target_role_ids.clone();
        let mut current_sorted = member.roles.clone();

        target_sorted.sort();
        current_sorted.sort();

        if target_sorted != current_sorted {
            role_editor_tasks.push(async move {

                let res = match member.edit(get_http(), EditMember::default()
                    .roles(target_role_ids)).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to edit roles of {}: {}", member.user.name, e)),
                };
                res
            });
        }
    }

    println!("{}", "Finished creating tasks to edit ranking roles".green());

    // errors get logged and ignored
    let editor_taks_results = futures::stream::iter(role_editor_tasks).buffer_unordered(5).collect::<Vec<_>>().await;
    let editor_tasks_errs = editor_taks_results.iter().filter(|res| res.is_err()).collect::<Vec<_>>();
    for err in editor_tasks_errs {
        if let Err(e) = err {
            println!("{}\n{}{}", "An error occurred while editing roles: ".red(), e.to_string().bright_red(), " - role editing was therefore skipped".yellow());
        }
    }

    println!("{}", "Finished editing ranking roles".green());
    
    Ok(())
}