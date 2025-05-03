use colored::Colorize;
use serenity::prelude::*;

use crate::backend::bot_communication::make_bot_request_match;
use crate::liberary::account_lib::match_event::match_event::{MatchEvent, MatchStatus};
use crate::liberary::account_lib::match_event::storage::get_match_events::get_match_events;
use crate::liberary::dialogue_lib::dialogue_builder::storage::get_dialogues::get_dialogues;
use crate::liberary::dialogue_lib::dialogue_plan::dialogue_data::CaseData;
use crate::liberary::matchplan_lib::matchplan::storage::get_seasons::get_seasons;
use crate::liberary::matchplan_lib::matchplan::storage::matchplan_get::get_matchplan;
use crate::AppState;


// Command to go through all match events to see if they have accoring dialogues and creates any missing ones
// TODO: unfinished and needs fixing, but isnt required for now and eats up my time
pub async fn match_request_catch_up(appstate: &AppState) -> Result<(), String> {

    println!("{}", "Received command to catch up with match requests".magenta());

    let dialogues = match get_dialogues(100000, appstate.pool.clone()).await {
        Ok(dialogues) => dialogues,
        Err(e) => return Err(format!("Error getting dialogues: {}", e))
    };

    let planed_matchevents: Vec<String> = dialogues.iter().filter_map(|dialogue| {
        match dialogue.dialogue_data.data.clone() {
            CaseData::MatchRequest(data) => {
                let matchevent_id = data.match_info.get_id();
                return Some(matchevent_id);
            },
            _ => return None
        }
    }).collect();

    let seasons = get_seasons(appstate.pool.clone()).await;
    let seasons = match seasons {
        Ok(seasons) => seasons,
        Err(e) => return Err(format!("Error getting seasons: {}", e))
    };

    let current_season = match seasons.first() {
        Some(season_name) => {*season_name},
        None => return Err("No active season found".to_string())
    };

    let matchplan = get_matchplan(current_season, appstate.pool.clone()).await;
    let matchplan = match matchplan {
        Ok(matchplan) => matchplan,
        Err(e) => return Err(format!("Error getting matchplan: {}", e))
    };

    let mut match_events = vec!();

    for player in matchplan.players.iter() {
        let player_match_events = match get_match_events(player.id, appstate.pool.clone()).await {
            Ok(player_match_events) => player_match_events,
            Err(e) => return Err(format!("Error getting match events for player {}: {}", player.id, e))
        };
        match_events.extend(player_match_events);
    }

    let mut filtered_match_events = vec!();
    for matchevent in match_events.iter() {
        if !filtered_match_events.contains(matchevent) && 
            matchevent.season == current_season {
            filtered_match_events.push(matchevent.clone());
        }
    }

    let matchevents_to_plan = filtered_match_events.iter().filter(|matchevent| {
        if let MatchStatus::Requested = matchevent.status {
            !planed_matchevents.contains(&id)
        } else {
            false
        }
    }).map(|(_id, matchevent)| matchevent.clone()).collect::<Vec<MatchEvent>>();

    
    let matchplan_clone = appstate.matchplan.clone();
    let matchplan_lock = matchplan_clone.lock().await;
    let matchplan_ref = matchplan_lock.as_ref();

    let matchplan = match matchplan_ref {
        Some(v) => v,
        _ => return Err("no matchplan could be found".to_string())
    };

    for matchevent in matchevents_to_plan.iter() {
        let mut league = None;
        for division in matchplan.divisions.iter() {
            for player in division.players.iter() {
                if (player.id == matchevent.challenger_id as i64 ||
                    player.id == matchevent.opponent_id as i64) {
                    league = Some(division.name.clone());
                }
            }
        }

        match league {
            Some(division_name) => {
                make_bot_request_match(matchevent.clone(), division_name, appstate).await?;
            },
            None => return Err("couldnt find division of all participants".to_string())
        }
    }

    drop(matchplan_lock);

    let _ = appstate.refresh_dialogues().await;
    Ok(())
}