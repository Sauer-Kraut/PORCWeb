use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::liberary::matchplan_lib::player::player::Player;
use crate::AppState;


impl MatchPlan {

    pub fn generate(blueprint: PlanBlueprint, allow_doubles: bool) -> Result<MatchPlan, DataGenerationError> {

        let mut entered_players: Vec<PlayerBlueprint> = vec!();
        let mut player_objects = vec!();
        let mut divisions = vec!();

        let mut end_timestamp = 0;

        match blueprint.end_timestamp {
            None => {return Err(DataGenerationError::MatchMapGenerationError(format!("No end timestamp provided :(")));},
            Some(v) => {end_timestamp = v}
        }

        let mut pause_end_timestamp = 0;

        match blueprint.pause_end_timestamp {
            None => {return Err(DataGenerationError::MatchMapGenerationError(format!("No pause end timestamp provided :(")));},
            Some(v) => {pause_end_timestamp = v}
        }

        for division_plan in blueprint.divisions.iter() {

            divisions.push(division_plan.name.clone());

            for (_index, player) in division_plan.players.iter().enumerate() {

                if entered_players.contains(&player) {
                    if allow_doubles {
                        println!("{}", "Warning: Player with already existing name has been entered into Matchplan".bold().red());
                    }

                    else {
                        return Err(DataGenerationError::MatchMapGenerationError(format!("Player of same name already exists: {:?}, doubles were disallowed", player)));
                    }            
                }

                entered_players.push(player.clone());

                let player_object = Player {
                    id: player.id.clone(),
                    tag: player.tag.clone(),
                    division: division_plan.name.clone(),
                };

                player_objects.push(player_object);
            }
        }
        
        let division_plan = Division::generate_set(player_objects.clone(), divisions)?;

        Ok(MatchPlan {
            divisions: division_plan,
            players: player_objects,
            end_timestamp,
            pause_end_timestamp,
            season: blueprint.season
        })
    }

    pub fn add_player(&mut self, add_player: (PlayerBlueprint, String), allow_doubles: bool) -> Result<(), DataGenerationError> {

        let (add_player, add_player_div)= add_player;

        for player in self.players.iter() {

            if add_player.tag == player.tag {

                if allow_doubles {
                    println!("{}", "Warning: Player with already existing name has been entered into Matchplan".bold().red());
                }

                else {
                    return Err(DataGenerationError::MatchMapAdditionError(format!("Player of same name already exists: {}, doubles were disallowed", player.tag)));
                }
            }
        }

        let add_player_object = Player {
            id: add_player.id,
            tag: add_player.tag,
            division: add_player_div.clone()
        };

        let mut entered = false;

        for division in self.divisions.iter_mut() {

            if division.name == add_player_div {

                division.add_players(vec!(add_player_object.clone()))?;
                self.players.push(add_player_object);
                entered = true;
                break;
            }
        }

        if !entered {
            return Err(DataGenerationError::MatchMapAdditionError(format!("Player target division does not exist: {}", add_player_div)));
        }

        Ok(())
    }

    pub fn remove_player(&mut self, remove_player: &Player) -> Result<(), DataGenerationError> {

        let player_index = self.players.iter().position(|player| player == remove_player);

        if let Some(index) = player_index {

            let player = self.players.remove(index);

            for division in self.divisions.iter_mut() {

                if division.name == player.division {
                    division.remove_players(vec![&player])?;
                    break;
                }
            }

            Ok(())
        } 
        
        else {
            Err(DataGenerationError::MatchMapRemovalError(format!("Player with tag {} does not exist", remove_player.tag)))
        }
    }

    pub fn move_player(&mut self, move_player: &mut Player, target_division: &str) -> Result<(), DataGenerationError> {

        let player_index = self.players.iter().position(|player| player == move_player);

        if let Some(index) = player_index {

            let player = self.players.remove(index);
            let mut removed = false;

            for division in self.divisions.iter_mut() {

                if division.name == move_player.division {
                    division.remove_players(vec![&player])?;
                    removed = true;
                    break;
                }
            }

            if !removed {
                return Err(DataGenerationError::MatchMapRemovalError(format!("Player wasnt found in any division: {:?}", move_player)))
            }

            move_player.division = target_division.to_string();

            for division in self.divisions.iter_mut() {

                if division.name == target_division {

                    match division.add_players(vec![player.clone()]) {
                        Err(err) => {

                            panic!("{}", format!("Critical error encountered while moving player when data was already modified. Thread paniced in order to prevent faulty saves. \nError: {:?}", err));
                        }
                        _ => {}
                    };
                    break;
                }
            }

            Ok(())
        }

        else {
            return Err(DataGenerationError::MatchMapGenerationError(format!("Player wasnt found in player list: {:?}", move_player)))
        }
    }

    pub fn update_match(&mut self, match_info: Match) -> Result<(), DataGenerationError> {

        for division in self.divisions.iter_mut() {

            for (_key, value) in division.matches.iter_mut() {

                if value.p1.id == match_info.p1.id && value.p2.id == match_info.p2.id {
                    *value = match_info.clone();
                    return Ok(());
                } else if value.p1.id == match_info.p2.id && value.p2.id == match_info.p1.id {
                    let fit_match_info = Match {
                        p1: match_info.p1.clone(),
                        p2: match_info.p2.clone(),
                        p1score: match_info.p2score,
                        p2score: match_info.p1score,
                    };
                    *value = fit_match_info;
                    return Ok(());
                } else {
                    // println!("Match info did not match: \n{:?} is not same as match info  \n{:?}", value, match_info);

                    // if value.p1.id != match_info.p1.id {
                    //     println!("ID of p1 does not match: p1 id: {}, p1 info id: {}", value.p1.id, match_info.p1.id);
                    // }
                }
            }
        }

        return Err(DataGenerationError::MatchMapModificationError("Match not found in any division".to_string()));
    }

    pub async fn refresh(&mut self, appstate: AppState) {

        let accounts_clone = appstate.accounts.clone();
        let accounts = accounts_clone.lock().await;

        for division in self.divisions.iter_mut() {

            for (_key, value) in division.matches.iter_mut() {

                let account_p1 = accounts.get(&value.p1.id);

                match account_p1 {
                    Some(account) => {
                        if account.user_info.username != value.p1.tag {
                            value.p1.tag = account.user_info.username.clone();
                        }
                    },
                    None => {},
                }

                let account_p2 = accounts.get(&value.p2.id);

                match account_p2 {
                    Some(account) => {
                        if account.user_info.username != value.p2.tag {
                            value.p2.tag = account.user_info.username.clone();
                        }
                    },
                    None => {},
                }
            }
        }
    }
}


impl Division {

    pub fn generate_set(players: Vec<Player>, divisions: Vec<String>) -> Result<Vec<Division>, DataGenerationError> {

        let mut output = vec!();
        let mut entered_players = vec!();

        for (index, division) in divisions.iter().enumerate() {

            let player_set: Vec<Player> = players.iter().filter(|player| player.division == *division).map(|player| player.clone()).collect();
            entered_players.append(&mut player_set.clone());

            if player_set.len() == 1 {
                return Err(DataGenerationError::MatchMapGenerationError(format!("Division {} has only one player", division)));
            }

            if player_set.len() == 0 {
                continue;
            }

            let division_map = Match::generate_matchmap(&player_set)?;

            let entry = Division {
                name: division.clone(),
                order: index,
                matches: division_map,
                players: player_set
            };

            output.push(entry);
        }

        if entered_players.len() != players.len() {
            let missing_players: Vec<_> = players.iter().filter(|player| !entered_players.contains(player)).collect();
            return Err(DataGenerationError::MatchMapAdditionError(format!("Some players were not assigned to any division: {:?}", missing_players)));
        }

        return Ok(output)
    }

    pub fn add_players(&mut self, mut add_player_set: Vec<Player>) -> Result<(), DataGenerationError> {

        for add_player in add_player_set.iter() {

            if self.players.contains(add_player) {
                return Err(DataGenerationError::MatchMapAdditionError("Player already in division".to_string()));
            }

            if add_player.division != self.name {
                return Err(DataGenerationError::MatchMapAdditionError("Player division does not allign with divison".to_string()));
            }
        }

        self.players.append(&mut add_player_set);

        let new_matchmap = Match::generate_matchmap(&self.players)?;

        for (key, value) in new_matchmap.iter() {

            self.matches.entry(key.clone()).or_insert(value.clone());
        }

        Ok(())

    }

    pub fn remove_players(&mut self, remove_player_set: Vec<&Player>) -> Result<(), DataGenerationError> {

        for remove_player in remove_player_set.iter() {

            if !self.players.contains(remove_player) {
                return Err(DataGenerationError::MatchMapRemovalError("Player not in division".to_string()));
            }

            if remove_player.division != self.name {
                return Err(DataGenerationError::MatchMapRemovalError("Player division does not align with division".to_string()));
            }
        }

        if self.players.len() - remove_player_set.len() < 2 {

            return Err(DataGenerationError::MatchMapRemovalError(format!("Division does not have enough players to loose {} players", remove_player_set.len())));
        }

        self.players.retain(|player| !remove_player_set.contains(&player));

        let new_matchmap = Match::generate_matchmap(&self.players)?;

        self.matches.retain(|key, _| new_matchmap.contains_key(key));

        Ok(())
    }

    pub async fn generate_perfomance(&self) -> Vec<PlayerPerformance> {

        let mut output = vec!();

        for player in self.players.iter() {

            let mut wins = 0;
            let mut matches = 0;
            let mut cumulative_match_difference = 0;

            for (_key, battle) in self.matches.iter() {

                if battle.p1 == *player || battle.p2 == *player {

                    if battle.p1score.is_some() && battle.p2score.is_some() {

                        if battle.p1 == *player {
                            cumulative_match_difference += battle.p1score.unwrap() - battle.p2score.unwrap();
                        }
                        else {
                            cumulative_match_difference += battle.p2score.unwrap() - battle.p1score.unwrap();
                        }
                        matches += 1;

                        if battle.p1 == *player && battle.p1score.unwrap() > battle.p2score.unwrap() {
                            wins += 1;
                        }

                        if battle.p2 == *player && battle.p2score.unwrap() > battle.p1score.unwrap() {
                            wins += 1;
                        }
                    }
                }
            }

            output.push(PlayerPerformance {
                player: player.clone(),
                wins,
                matches,
                cumulative_match_difference
            });
        }

        output.sort_by(|a, b| b.partial_cmp(a).unwrap());

        output
    }
    
}



impl Match {

    fn generate_matchmap(player_set: &Vec<Player>) -> Result<HashMap<String, Match>, DataGenerationError> {
        
        if player_set.len() < 2 {
            return Err(DataGenerationError::MatchMapGenerationError("Too few players :(".to_string()));
        }

        let mut division_map = HashMap::new();

        for player in player_set.iter() {
            for opponent in player_set.iter() {
                if player.id == opponent.id {
                    continue;
                }

                let mut lower_id_player = player;
                let mut higher_id_player = opponent;

                if lower_id_player.id.parse::<i64>().unwrap() > higher_id_player.id.parse::<i64>().unwrap() {
                    lower_id_player = opponent;
                    higher_id_player = player;
                }

                let match_key = format!("{}V{}", lower_id_player.id, higher_id_player.id);
                let match_info = Match {
                    p1: lower_id_player.clone(),
                    p2: higher_id_player.clone(),
                    p1score: None,
                    p2score: None,
                };

                let _ = division_map.insert(match_key, match_info);
            }
        }

        Ok(division_map)
    }
}



#[derive(Debug)]
pub enum DataGenerationError {
    DivisionGenerationError(String),
    MatchMapGenerationError(String),
    MatchMapAdditionError(String),
    MatchMapRemovalError(String),
    MatchMapModificationError(String),
}

impl fmt::Display for DataGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataGenerationError::DivisionGenerationError(err) => write!(f, "Division generation Error: {}", err),
            DataGenerationError::MatchMapGenerationError(err) => write!(f, "Match Map generation Error: {}", err),
            DataGenerationError::MatchMapAdditionError(err) => write!(f, "Division player addition Error: {}", err),
            DataGenerationError::MatchMapRemovalError(err) => write!(f, "Division player removal Error: {}", err),
            DataGenerationError::MatchMapModificationError(err) => write!(f, "Division match modification Error: {}", err),
        }
    }
}

impl std::error::Error for DataGenerationError {}