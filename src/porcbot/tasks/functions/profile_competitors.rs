use std::num::NonZero;

use colored::Colorize;
use serenity::all::{GuildId, Member, RoleId};

use crate::{liberary::{account_lib::account::{account::Account, discord_user::DiscordUser, storage::create_account::create_account}, dialogue_lib::{bot_error::BotError, dialogue_builder::storage::{get_dialogues::get_dialogues, store_dialogue::store_dialogue}}}, porcbot::config::{get_http, RANKS, SERVER_ID}, AppState};
use crate::liberary::matchplan_lib::matchplan::matchplan::MatchPlan;

pub async fn profile_competitors(appstate: &AppState) -> Result<(), BotError> {

    let guild_id = GuildId::new(SERVER_ID.as_ref().clone());
    let members = guild_id.members(get_http(), None, None).await?;

    let competitor_role = guild_id.roles(get_http()).await?
        .into_iter()
        .find(|(_, r)| r.name == "Competitor")
        .map(|(id, _)| id)
        .ok_or_else(|| BotError::LogicError("Competitor role not found".to_owned().into()))?;

    let competitors = members.iter()
        .filter(|m| m.roles.iter().any(|r| *r == competitor_role))
        .collect::<Vec<&Member>>();


    for competitor in competitors {
        let new_account = Account {
            user_info: DiscordUser {
                id: competitor.user.id.to_string(),
                username: competitor.user.name.clone(),
                discriminator: competitor.user.discriminator.map(|v| v.to_string()).unwrap_or("0".to_string()),
                avatar: competitor.user.avatar.map(|v| v.to_string()),
                email: competitor.user.email.clone(),
            },
            schedule: None,
        };

        // does nothing if an account already exists
        let _res = create_account(new_account, appstate.pool.clone()).await?;
    }
    
    Ok(())
}