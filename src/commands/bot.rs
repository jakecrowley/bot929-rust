use poise::command;
use poise::serenity_prelude::{User, UserId, GuildId};

use crate::prelude::{BotContext, BotResult};


#[command(prefix_command)]
pub async fn profile(
    ctx: BotContext<'_>,
    #[description = "Retrieve you or someone else's 929 profile."] user: Option<User>,
    
) -> BotResult<()> {
    let user_id: UserId = user.as_ref().unwrap_or(ctx.author()).id;
    let guild_id: GuildId = ctx.guild_id().unwrap();

    println!("user executed profile command");

    ctx.send(|response: &mut poise::CreateReply<'_>| response.content(format!("<@{}> deez nuts lmao", user_id)).reply(true)).await?;

    Ok(())
}