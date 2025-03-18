use serenity::all::*;
use rand::Rng;

use crate::porcbot::config::*;
use crate::backend::account_lib::MatchEvent;

pub async fn create_discord_event(match_event: MatchEvent, league: String) -> Result<ScheduledEvent, String>{
    let guild_id = GuildId::new(SERVER_ID);
    let initiator_username = match UserId::new(match match_event.initiator_id.parse() {Ok(v) => v, Err(err) => return Err("couldnt parse initiator id into u64".to_string())}).to_user(get_http()).await {
        Ok(user) => user.name,
        Err(_) => "Challenger".to_string(),
    };

    let opponent_username = match UserId::new(match match_event.opponent_id.parse() {Ok(v) => v, Err(err) => return Err("couldnt parse opponent id into u64".to_string())}).to_user(get_http()).await {
        Ok(user) => user.name,
        Err(_) => "Opponent".to_string(),
    };

    let channel_index = rand::thread_rng().gen_range(0..2);
    let channels = vec![STAGE_IDS.0, STAGE_IDS.1, STAGE_IDS.2];
    let channel_id = channels.get(channel_index).unwrap();

    let event_name = format!("[{league}] {initiator_username} vs. {opponent_username}");

    let start_timestamp = Timestamp::from_unix_timestamp(match_event.start_timestamp as i64).unwrap();

    match guild_id.create_scheduled_event(get_http(), CreateScheduledEvent::new(
        ScheduledEventType::StageInstance, 
        event_name, 
        start_timestamp
    ).channel_id(channel_id.clone())).await {
        Ok(v) => return Ok(v),
        Err(err) => return Err(format!("failed to create event with error: {:?}", err))
    }

}