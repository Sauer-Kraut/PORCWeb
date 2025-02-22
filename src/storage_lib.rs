use crate::account_lib::{Account, DiscordUser};
use crate::{discord_communication, MatchPlan, SignUpInfo};
use askama::filters::format;
use discord_communication::TokenRequestParam;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write, self};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub match_plan: Option<MatchPlan>,
    pub sign_ups: Vec<SignUpInfo>,
    pub season: usize
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub url: String,
    pub domain: String,
    pub port: String
}

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

    pub fn save_logins(logins: HashMap<String, String>, path: &str) -> Result<(), io::Error> {
        let json = serde_json::to_string_pretty(&logins)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn read_logins(path: &str) -> Result<HashMap<String, String>, io::Error> {
        let mut file = File::open(path)?;
        let mut json = "".to_string();
        file.read_to_string(&mut json)?;
        let signups: HashMap<String, String> = serde_json::from_str(&json)?;
        Ok(signups)
    }

    pub fn save_accounts(accounts: HashMap<String, Account>, path: &str) -> Result<(), io::Error> {
        let json = serde_json::to_string_pretty(&accounts)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn read_accounts(path: &str) -> Result<HashMap<String, Account>, io::Error> {
        let mut file = File::open(path)?;
        let mut json = "".to_string();
        file.read_to_string(&mut json)?;
        let accounts: HashMap<String, Account> = serde_json::from_str(&json)?;
        Ok(accounts)
    }

    pub fn read_secrets() -> Result<TokenRequestParam, io::Error> {
        let path = "secrets.json";
        let mut file = File::open(path)?;
        let mut json = "".to_string();
        file.read_to_string(&mut json)?;
        let info: TokenRequestParam = serde_json::from_str(&json)?;
        Ok(info)
    }

    pub fn save_record(record: Record, directory: &str) -> Result<(), io::Error> {
        let json = serde_json::to_string_pretty(&record)?;

        let file_path = format!("{}/Season{}Record.json", directory, record.season);

        let mut file = File::create(&file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn read_config() -> Result<Config, io::Error> {
        let path = "config.json";
        let mut file = File::open(path)?;
        let mut json = "".to_string();
        file.read_to_string(&mut json)?;
        let info: Config = serde_json::from_str(&json)?;
        Ok(info)
    }

}