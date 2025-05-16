use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::liberary::matchplan_lib::matchplan_match::matchplan_match::Match;
use crate::liberary::matchplan_lib::player::player::Player;

use super::player_performance::PlayerPerformance;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Division {
    pub name: String,
    pub order: usize,
    pub matches: HashMap<String, Match>,
    pub players: Vec<Player>,
}

impl fmt::Display for Division {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Division: {}\nOrder: {}\nMatches:\n", self.name, self.order)?;
        for (key, match_) in &self.matches {
            write!(f, "{}: {}\n", key, match_)?;
        }
        write!(f, "Players:\n")?;
        for player in &self.players {
            write!(f, "{}\n", player)?;
        }
        Ok(())
    }
}

impl Division {
    pub async fn generate_perfomance(&self) -> Vec<PlayerPerformance> {

        let mut output = vec!();

        for player in self.players.iter() {

            let mut wins = 0;
            let mut matches = 0;
            let mut cumulative_match_difference = 0;

            for (_key, battle) in self.matches.iter() {

                if battle.p1 == *player || battle.p2 == *player {

                    if battle.p1score.is_some() && battle.p2score.is_some() {

                        matches += 1;
                        if battle.p1 == *player {
                            cumulative_match_difference += battle.p1score.unwrap() as isize - battle.p2score.unwrap() as isize;
                            }
                        else {
                            cumulative_match_difference += battle.p2score.unwrap() as isize - battle.p1score.unwrap() as isize;
                            }

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
                cumulative_match_difference,
            });
        }

        output.sort_by(|a, b| b.partial_cmp(a).unwrap());

        output
    }
}