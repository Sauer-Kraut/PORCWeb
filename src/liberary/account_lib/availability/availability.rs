use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Availability {
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub repetition: Repetition,
    pub repetition_config: Option<DailyRepetitionConfig>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Repetition {
    Once,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl Repetition {
    pub fn from_type_code(code: i16) -> Result<Self, String> {
        match code {
            0 => Ok(Repetition::Once),
            1 => Ok(Repetition::Daily),
            2 => Ok(Repetition::Weekly),
            3 => Ok(Repetition::Monthly),
            4 => Ok(Repetition::Yearly),
            _ => Err(format!("Invalid repetition type code: {}", code)),
        }
    }

    pub fn to_type_code(&self) -> i16 {
        match self {
            Repetition::Once => 0,
            Repetition::Daily => 1,
            Repetition::Weekly => 2,
            Repetition::Monthly => 3,
            Repetition::Yearly => 4,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DailyRepetitionConfig {
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}