use bson::{doc, Bson};
use mongodm::f;

use poise::command;
use poise::serenity_prelude::{User, CreateEmbed, GuildId, CacheHttp, Color, ButtonStyle, ReactionType};

use crate::prelude::utils::get_leaderboard;
use crate::prelude::{BotContext, BotResult};
use crate::Nine92er;

use crate::prelude::utils::CHANNEL_CONF;

#[command(prefix_command)]
pub async fn profile(
    ctx: BotContext<'_>,
    #[description = "Retrieve your or someone else's 929 profile."] user: Option<User>,
) -> BotResult<()> {
    if ctx.channel_id().0 != CHANNEL_CONF.channel_id {
        return Ok(())
    }

    let user: &User = user.as_ref().unwrap_or(ctx.author());
    let guild_id: GuildId = ctx.guild_id().unwrap();

    let nickname: String = user.nick_in(&ctx.http(), guild_id).await.unwrap_or(user.name.clone());

    let user_id_bson: Bson = Bson::Int64(user.id.0 as i64);
    let profile: Option<Nine92er> = ctx.data().nine29ers.find_one(doc! { f!(_id in Nine92er): user_id_bson }, None).await?;
    
    match profile {
        Some(profile) => {
            let msg = ctx.send(|resp| 
                resp.embed(|e: &mut CreateEmbed| {
                    e.title(format!("Profile for {}", nickname))
                     .fields(vec![
                        ("Current Streak", profile.currentstreak, false),
                        ("Longest Streak", profile.maxstreak, false),
                     ])
                     .field("Total Points", profile.points, false)
                     .field("Total 929s", profile.count, false)
                }).reply(true)
            ).await;


            if let Err(why) = msg {
                log::error!("Error sending message: {:?}", why);
            }
        },
        None => {
            ctx.send(|response: &mut poise::CreateReply<'_>| response.content("You have not participated in a 9:29 yet!").reply(true)).await?;
        }
    };

    Ok(())
}

#[command(prefix_command)]
pub async fn leaderboard(ctx: BotContext<'_>) -> BotResult<()> {
    if ctx.channel_id().0 != CHANNEL_CONF.channel_id {
        return Ok(())
    }

    let leaderboard_str: String = get_leaderboard(ctx.data(), ctx.http(), 0).await?;

    let _msg = ctx.send(|resp| 
        resp.embed(|e: &mut CreateEmbed| {
            e.title("Leaderboard")
             .description(leaderboard_str)
             .color(Color::DARK_RED)
        }).components(|c| {
            c.create_action_row(|r| {
                r.create_button(|b| {
                    b.style(ButtonStyle::Primary).custom_id("prev").label("").emoji(ReactionType::Unicode("⬅️".to_string()))
                }).create_button(|b| {
                    b.style(ButtonStyle::Primary).custom_id("next").label("").emoji(ReactionType::Unicode("➡️".to_string()))
                })
            })
        }).reply(true)
    ).await?;

    Ok(())
}