use colored::Colorize;
use serenity::prelude::*;

use crate::backend::account_lib::{MatchEvent, MatchStatus};
use crate::backend::bot_communication::make_bot_request_match;
use crate::porcbot::dialogue_module::dialogue_data::CaseData;
use crate::AppState;

pub async fn match_request_catch_up(appstate: &AppState) -> Result<(), String> {

    println!("{}", "Received command to catch up with match requests".magenta());

    let dialogues_clone = appstate.dialogues.clone();
    let dialogues_lock = dialogues_clone.lock().await;
    let dialogues = dialogues_lock.clone();
    drop(dialogues_lock);

    let planed_matchevents: Vec<String> = dialogues.iter().filter_map(|dialogue| {
        match dialogue.dialogue_data.data.clone() {
            CaseData::MatchRequest(data) => {
                let matchevent_id = data.match_info.get_id();
                return Some(matchevent_id);
            },
            _ => return None
        }
    }).collect();
    
    let matchevents_clone = appstate.matchevents.clone();
    let matchevents_lock = matchevents_clone.lock().await;
    let matchevents = matchevents_lock.clone();
    drop(matchevents_lock);

    let matchevents_to_plan = matchevents.iter().filter(|(id, matchevent)| {
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
                if (player.id == matchevent.initiator_id ||
                    player.id == matchevent.opponent_id) {
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