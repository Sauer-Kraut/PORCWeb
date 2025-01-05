use crate::{MatchPlan, SignUpInfo};
use std::fs::File;
use std::io::{Read, Write, self};


pub struct StorageMod {}

impl StorageMod {

    pub fn save_matchplan(plan: MatchPlan, path: &str) -> Result<(), io::Error> {
        let json = serde_json::to_string_pretty(&plan)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn read_matchplan(path: &str) -> Result<MatchPlan, io::Error> {
        let mut file = File::open(path)?;
        let mut json = "".to_string();
        file.read_to_string(&mut json)?;
        let plan: MatchPlan = serde_json::from_str(&json)?;
        Ok(plan)
    }

    pub fn save_signups(signups: Vec<SignUpInfo>, path: &str) -> Result<(), io::Error> {
        let json = serde_json::to_string_pretty(&signups)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn read_signups(path: &str) -> Result<Vec<SignUpInfo>, io::Error> {
        let mut file = File::open(path)?;
        let mut json = "".to_string();
        file.read_to_string(&mut json)?;
        let signups: Vec<SignUpInfo> = serde_json::from_str(&json)?;
        Ok(signups)
    }

}