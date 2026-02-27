use colored::Colorize;
use serenity::all::{ChannelId, CreateAttachment, CreateMessage, UserId};

use crate::{AppState, liberary::{account_lib::signup::{signup::SignUpInfo, storage::get_signups::get_signups}, dialogue_lib::bot_error::BotError, matchplan_lib::matchplan::storage::matchplan_get::get_matchplan}, porcbot::{config::get_http, tasks::functions::update_ranking_roles::update_ranking_roles, util::dm_send_with_attachment::send_dm_with_attachment}};
use serde::Serialize;


#[derive(Serialize)]
struct ReadableSignup {
    name: String,
    bp: String,
    region: String
}

impl SignUpInfo {
    fn to_readable(self) -> ReadableSignup {
        ReadableSignup { name: self.username, bp: self.bp.to_string(), region: self.region }
    }
}

fn format_signup_json(list: Vec<ReadableSignup>) -> String {

    // Compute max width for alignment
    let max_id_width = list
        .iter()
        .map(|p| p.name.to_string().len() + 1)
        .max()
        .unwrap();

    let rows: Vec<String> = list.iter().map(|p| {
        let region = match p.region.as_str() {"NaN" => "__".to_string(), v => v.to_string()}.drain(0..2).collect::<String>();
        format!(
            "  {{ \"id\": {:<width$}, \"name\": \"{:<width$}\", \"region\": \"{}\" }}",
            p.name,
            p.bp,
            region,
            width = max_id_width
        )
    }).collect();

    let json = format!("[\n{}\n]", rows.join(",\n"));

    json
}


pub async fn get_season_signups(appstate: &AppState, channel: ChannelId) -> Result<(), BotError> {

    println!("{}", "Received command to fetch season signups".magenta());

    
    let season_opt = appstate.season.read().await.clone();
    let season = match season_opt {
        Some(season) => season,
        None => {
            return Err(BotError::LogicError("No current season found".to_owned().into()));
        }
    };

    let signups = get_signups(season.start_timestamp, None, appstate.pool.clone()).await?;
    let count = signups.len();

    let value = format_signup_json(signups.iter().map(|s| s.clone().to_readable()).collect::<Vec<ReadableSignup>>());

    let content = format!("**{} Signups** since start of {}", count, season.name);
    let attachment_bytes = value.into_bytes();
    let create_attachment = CreateAttachment::bytes(attachment_bytes, format!("PORC_p{}_signups.json", season.name.clone()));

    let _message = channel.send_message(get_http(), CreateMessage::new().content(content).add_file(create_attachment)).await?;
    return Ok(())
}