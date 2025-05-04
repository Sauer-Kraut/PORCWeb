use serde::{Deserialize, Serialize};

use crate::liberary::account_lib::availability::availability::Availability;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub availabilities: Vec<Availability>,
    pub matches: Vec<i32>, // list of match ids, only meant for reading
    pub note: String
}