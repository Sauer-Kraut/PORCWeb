use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: u64,
    pub tag: String,
    pub division: String
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player {} (ID: {}) in division {}", self.tag, self.id, self.division)
    }
}