use std::{fmt::format, num::NonZero};

use colored::Colorize;
use serenity::{all::{ChannelId, GetMessages, GuildId, Member, RoleId}, model::guild};

use crate::{AppState, liberary::{account_lib::account::{pub_account_info, storage::get_account::get_account}, dialogue_lib::bot_error::BotError, discord_lib::discord_event::discord_event::DiscordEvent}, porcbot::config::{RANKS, SERVER_ID, VOD_FORUM_ID, get_http}};
use crate::liberary::discord_lib::video_reference::video_reference::VideoReference;

pub async fn collect_discord_vods(appstate: &AppState, limit: usize) -> Result<Vec<VideoReference>, BotError> {

    let guild_id = GuildId::new(SERVER_ID.as_ref().clone());
    let forum = ChannelId::new(VOD_FORUM_ID.as_ref().clone());
    

    let posts = guild_id.get_active_threads(get_http()).await?.threads.into_iter().filter(|thread| thread.parent_id == Some(forum)).take(limit).collect::<Vec<_>>();
    let mut vods = vec!();

    for post in posts.iter() {
        // println!("{} {}, {}", "Found vod post:".green(), post.name.clone(), post.topic.clone().unwrap_or_default());

        let messages = post.messages(get_http(), GetMessages::default().limit(10)).await?;
        let mut video_id = "";

        // isolating the youtube video id from the first youtube link in the first message that contains one
        for message in messages.iter() {
            if message.content.contains("youtube.com") || message.content.contains("youtu.be") {
                match message.content.split_whitespace().filter_map(|part| {
                    if part.contains("youtube.com") || part.contains("youtu.be") {
                        part.split(|c| c == '/').filter(|spl| !spl.contains("http") && !spl.contains("youtube.com") && !spl.contains("youtu.be")).filter_map(|spl| {
                            let cleaned = spl
                                .split(|c| c == '=' || c == '&')   // drop any query string
                                .filter(|s| !s.contains("watch"))
                                .next()
                                .unwrap_or(spl);

                            if !cleaned.is_empty() {
                                Some(cleaned)
                            } else {
                                None
                            }
                        }).next()
                    } else {
                        None
                    }
                }).next() {
                    Some(id) => {
                        video_id = id;
                        break;
                    },
                    None => continue
                }
            }
        }


        let title = post.name.clone().replace('(', "[").replace(')', "]");

        let first_msg = match messages.first() {
            Some(msg) => msg,
            None => continue,
        };

        let vod = VideoReference {
            title: title,
            youtube_id: video_id.to_string(),
            creator: get_account(first_msg.author.id.to_string(), appstate.pool.clone()).await?.get_pub_info(),
            date: first_msg.timestamp.timestamp() as u64
        };

        vods.push(vod);
    }
    
    Ok(vods)
}