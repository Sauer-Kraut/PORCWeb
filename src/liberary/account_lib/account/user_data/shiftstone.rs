use serde::{Serialize, Deserialize};
use std::fmt::{self, Display};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Shiftstone {
    Stubborn,
    Surge,
    Adamant,
    Flow,
    Guard,
    Vigor,
    Volatile,
    Charge,
    Void,
    MisteriousHoldRelatedStone
}

impl From<String> for Shiftstone {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Stubborn" => Shiftstone::Stubborn,
            "Surge" => Shiftstone::Surge,
            "Adamant" => Shiftstone::Adamant,
            "Flow" => Shiftstone::Flow,
            "Guard" => Shiftstone::Guard,
            "Vigor" => Shiftstone::Vigor,
            "Volatile" => Shiftstone::Volatile,
            "Charge" => Shiftstone::Charge,
            "Void" => Shiftstone::Void,
            _ => Shiftstone::MisteriousHoldRelatedStone
        }
    }
}

impl From<i32> for Shiftstone {
    fn from(s: i32) -> Self {
        match s {
            0 => Shiftstone::Stubborn,
            1 => Shiftstone::Surge,
            2 => Shiftstone::Adamant,
            3 => Shiftstone::Flow,
            4 => Shiftstone::Guard,
            5 => Shiftstone::Vigor,
            6 => Shiftstone::Volatile,
            7 => Shiftstone::Charge,
            8 => Shiftstone::Void,
            _ => Shiftstone::MisteriousHoldRelatedStone
        }
    }
}

impl Display for Shiftstone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Shiftstone::Stubborn => "Stubborn",
            Shiftstone::Surge => "Surge",
            Shiftstone::Adamant => "Adamant",
            Shiftstone::Flow => "Flow",
            Shiftstone::Guard => "Guard",
            Shiftstone::Vigor => "Vigor",
            Shiftstone::Volatile => "Volatile",
            Shiftstone::Charge => "Charge",
            Shiftstone::Void => "Void",
            Shiftstone::MisteriousHoldRelatedStone => "MisteriousHoldRelatedStone"
        })?;
        Ok(())
    }
}

impl Shiftstone {
    fn to_code(&self) -> i32 {
        match self {
            Shiftstone::Stubborn => 0,
            Shiftstone::Surge => 1,
            Shiftstone::Adamant => 2,
            Shiftstone::Flow => 3,
            Shiftstone::Guard => 4,
            Shiftstone::Vigor => 5,
            Shiftstone::Volatile => 6,
            Shiftstone::Charge => 7,
            Shiftstone::Void => 8,
            Shiftstone::MisteriousHoldRelatedStone => -1
        }
    }
}