use super::discord_communication;
use discord_communication::TokenRequestParam;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, self};

// hardcoded to avoid incorect uses
const MATCHPLAN_PATH: &str = "userdata/SeasonMatchPlan.json";
const SIGNUPS_PATH: &str = "userdata/SeasonSignUps.json";
const LOGINS_PATH: &str = "userdata/DiscordLogIns.json";
const ACCOUNTS_PATH: &str = "userdata/UserAccounts.json";
const SECRETS_PATH: &str = "secrets.json";
const RECORDINGS_PATH: &str = "static/records";
const CONFIG_PATH: &str = "config.json";
const MATCHEVENTS_PATH: &str = "userdata/SeasonMatches.json";
const DIALOGUES_PATH: &str = "userdata/ActiveDialogues.json";



// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Record {
//     pub match_plan: Option<MatchPlan>,
//     pub sign_ups: Vec<SignUpInfo>,
//     pub season: usize
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub url: String,
    pub domain: String,
    pub port: String
}

pub struct StorageMod {}

impl StorageMod {

    // pub fn save_matchplan(plan: MatchPlan) -> Result<(), io::Error> {
    //     let path = MATCHPLAN_PATH;
    //     let json = serde_json::to_string_pretty(&plan)?;
    //     let mut file = File::create(path)?;
    //     file.write_all(json.as_bytes())?;
    //     Ok(())
    // }

    // pub fn read_matchplan() -> Result<MatchPlan, io::Error> {
    //     let path = MATCHPLAN_PATH;
    //     let mut file = File::open(path)?;
    //     let mut json = "".to_string();
    //     file.read_to_string(&mut json)?;
    //     let plan: MatchPlan = serde_json::from_str(&json)?;
    //     Ok(plan)
    // }

    // pub fn save_signups(signups: Vec<SignUpInfo>) -> Result<(), io::Error> {
    //     let path = SIGNUPS_PATH;
    //     let json = serde_json::to_string_pretty(&signups)?;
    //     let mut file = File::create(path)?;
    //     file.write_all(json.as_bytes())?;
    //     Ok(())
    // }

    // pub fn read_signups() -> Result<Vec<SignUpInfo>, io::Error> {
    //     let path = SIGNUPS_PATH;
    //     let mut file = File::open(path)?;
    //     let mut json = "".to_string();
    //     file.read_to_string(&mut json)?;
    //     let signups: Vec<SignUpInfo> = serde_json::from_str(&json)?;
    //     Ok(signups)
    // }

    // pub fn save_logins(logins: HashMap<String, String>) -> Result<(), io::Error> {
    //     let path = LOGINS_PATH;
    //     let json = serde_json::to_string_pretty(&logins)?;
    //     let mut file = File::create(path)?;
    //     file.write_all(json.as_bytes())?;
    //     Ok(())
    // }

    // pub fn read_logins() -> Result<HashMap<String, String>, io::Error> {
    //     let path = LOGINS_PATH;
    //     let mut file = File::open(path)?;
    //     let mut json = "".to_string();
    //     file.read_to_string(&mut json)?;
    //     let signups: HashMap<String, String> = serde_json::from_str(&json)?;
    //     Ok(signups)
    // }

    // pub fn save_accounts(accounts: HashMap<String, Account>) -> Result<(), io::Error> {
    //     let path = ACCOUNTS_PATH;
    //     let json = serde_json::to_string_pretty(&accounts)?;
    //     let mut file = File::create(path)?;
    //     file.write_all(json.as_bytes())?;
    //     Ok(())
    // }

    // pub fn read_accounts() -> Result<HashMap<String, Account>, io::Error> {
    //     let path = ACCOUNTS_PATH;
    //     let mut file = File::open(path)?;
    //     let mut json = "".to_string();
    //     file.read_to_string(&mut json)?;
    //     let accounts: HashMap<String, Account> = serde_json::from_str(&json)?;
    //     Ok(accounts)
    // }

    pub fn read_secrets() -> Result<TokenRequestParam, io::Error> {
        let path = SECRETS_PATH;
        let mut file = File::open(path)?;
        let mut json = "".to_string();
        file.read_to_string(&mut json)?;
        let info: TokenRequestParam = serde_json::from_str(&json)?;
        Ok(info)
    }

    // pub fn save_record(record: Record) -> Result<(), io::Error> {
    //     let json = serde_json::to_string_pretty(&record)?;
    //     let directory = RECORDINGS_PATH;

    //     let file_path = format!("{}/Season{}Record.json", directory, record.season);

    //     let mut file = File::create(&file_path)?;
    //     file.write_all(json.as_bytes())?;
    //     Ok(())
    // }

    pub fn read_config() -> Result<Config, io::Error> {
        let path = CONFIG_PATH;
        let mut file = File::open(path)?;
        let mut json = "".to_string();
        file.read_to_string(&mut json)?;
        let info: Config = serde_json::from_str(&json)?;
        Ok(info)
    }

    // pub fn save_matchevents(matchevents: HashMap<String, MatchEvent>) -> Result<(), io::Error> {
    //     let path = MATCHEVENTS_PATH;
    //     let json = serde_json::to_string_pretty(&matchevents)?;
    //     let mut file = File::create(path)?;
    //     file.write_all(json.as_bytes())?;
    //     Ok(())
    // }

    // pub fn read_matchevents() -> Result<HashMap<String, MatchEvent>, io::Error> {
    //     let path = MATCHEVENTS_PATH;
    //     let mut file = File::open(path)?;
    //     let mut json = "".to_string();
    //     file.read_to_string(&mut json)?;
    //     let matchevents: HashMap<String, MatchEvent> = serde_json::from_str(&json)?;
    //     Ok(matchevents)
    // }
    
    // pub fn save_dialogues(dialogues: Vec<DialogueBuilder>) -> Result<(), io::Error> {
    //     let path = DIALOGUES_PATH;
    //     let json = serde_json::to_string_pretty(&dialogues)?;
    //     let mut file = File::create(path)?;
    //     file.write_all(json.as_bytes())?;
    //     Ok(())
    // }

    // pub fn read_dialogues() -> Result<Vec<DialogueBuilder>, io::Error> {
    //     let path = DIALOGUES_PATH;
    //     let mut file = File::open(path)?;
    //     let mut json = "".to_string();
    //     file.read_to_string(&mut json)?;
    //     let dialogues: Vec<DialogueBuilder> = serde_json::from_str(&json)?;
    //     Ok(dialogues)
    // }

}