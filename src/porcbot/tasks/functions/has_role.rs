use serenity::all::*;

pub async fn has_role(ctx: &Context, guild_id: GuildId, user_id: UserId, role_name: &str) -> bool {
    if let Ok(member) = guild_id.member(&ctx.http, user_id).await {
        if let Ok(roles) = guild_id.roles(&ctx.http).await {
            return member.roles.iter().any(|role_id| {
                roles.get(role_id).map(|role| role.name == role_name).unwrap_or(false)
            });
        }
    }
    false
}


pub async fn has_role_from_message(ctx: &Context, msg: &Message, role_name: &str) -> bool {
    match msg.guild_id {
        Some(guild_id) => return  has_role(ctx, guild_id, msg.author.id, role_name).await,
        None => return false,
    }
}