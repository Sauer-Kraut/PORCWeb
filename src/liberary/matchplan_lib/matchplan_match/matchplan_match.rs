use serde::{Deserialize, Serialize};
use std::fmt;

use crate::liberary::matchplan_lib::player::player::Player;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Match {
    pub p1: Player,
    pub p2: Player,
    pub p1score: Option<usize>,
    pub p2score: Option<usize>,
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Match between {} and {}: {} - {}",
            self.p1.tag,
            self.p2.tag,
            self.p1score.map_or("None".to_string(), |s| s.to_string()),
            self.p2score.map_or("None".to_string(), |s| s.to_string())
        )
    }
}

impl Match {
    pub fn generate_key(&self) -> String {
        let mut lower_id_player = self.p1.clone();
        let mut higher_id_player = self.p2.clone();

        if lower_id_player.id > higher_id_player.id {
            lower_id_player = self.p2.clone();
            higher_id_player = self.p1.clone();
        }

        format!("{}V{}", lower_id_player.id, higher_id_player.id)
    }
}