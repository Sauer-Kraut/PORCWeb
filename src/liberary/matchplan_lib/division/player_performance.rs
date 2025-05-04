use serde::{Deserialize, Serialize};
use std::fmt;

use crate::liberary::matchplan_lib::player::player::Player;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerPerformance {
    pub player: Player,
    pub wins: usize,
    pub matches: usize,
    pub rounds: usize
    // TODO: add normalized round win field
}

impl fmt::Display for PlayerPerformance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player {} (ID: {}) in division {} with {} wins in {} matches and {} rounds", self.player.tag, self.player.id, self.player.division, self.wins, self.matches, self.rounds)
    }
}

impl PartialEq for PlayerPerformance {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player && self.wins == other.wins && self.matches == other.matches && self.rounds == other.rounds
    }
}

impl PartialOrd for PlayerPerformance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut ordering = self.wins.partial_cmp(&other.wins);

        if ordering != None {
            return ordering;
        }

        ordering = self.matches.partial_cmp(&other.matches);

        if ordering != None {
            return ordering;
        }

        self.rounds.partial_cmp(&other.rounds)
    }
}