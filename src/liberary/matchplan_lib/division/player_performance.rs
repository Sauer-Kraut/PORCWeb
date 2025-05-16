use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt};

use crate::liberary::matchplan_lib::player::player::Player;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerPerformance {
    pub player: Player,
    pub wins: usize,
    pub matches: usize,
    pub cumulative_match_difference: isize,
    // TODO: add normalized round win field
}

impl fmt::Display for PlayerPerformance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player {} (ID: {}) in division {} with {} wins in {} matches and a cumulative match difference of {}", self.player.tag, self.player.id, self.player.division, self.wins, self.matches, self.cumulative_match_difference)
    }
}

impl PartialEq for PlayerPerformance {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player && self.wins == other.wins && self.matches == other.matches && self.cumulative_match_difference == other.cumulative_match_difference
    }
}

impl PartialOrd for PlayerPerformance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut ordering = self.wins.partial_cmp(&other.wins);

        if ordering.unwrap() != Ordering::Equal {
            return ordering;
        }

        ordering = self.matches.partial_cmp(&other.matches);

        if ordering.unwrap() != Ordering::Equal {
            return ordering;
        }

        self.cumulative_match_difference.partial_cmp(&other.cumulative_match_difference)
    }
}