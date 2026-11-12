use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AvailabilityException {
    pub start_timestamp: u64,
    pub end_timestamp: u64
}